use crate::{
    extractors::authentication_token::{AuthenticationToken, Claims},
    models::{NewSession, NewUser, Role, Session, User},
    AppState,
};
use actix_web::{web, Error, HttpResponse, Scope};
use chrono::{Duration, Utc};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/sign-up", web::post().to(sign_up))
        .route("/sign-in", web::post().to(sign_in))
        .route("/decode-token", web::post().to(decode_token))
        .route("/protected", web::post().to(protected))
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct EncodeBody {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct EncodeResponse {
    message: String,
    token: String,
}

async fn sign_up(
    body: web::Json<EncodeBody>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut rng = rand::thread_rng();
    let id: usize = rng.gen();
    let exp: usize = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.secret.as_str().as_ref()),
    )
    .unwrap();

    let pool_clone = state.pool.clone();
    let body_clone = body.clone();
    web::block(move || {
        let mut conn = pool_clone.get()?;
        check_user(body_clone.email, &mut conn, true)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let pool_clone = state.pool.clone();
    let role = web::block(move || {
        let mut conn = pool_clone.get()?;
        get_admin(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut role_id = role[0].id.clone();

    let pool_clone = state.pool.clone();
    let user = web::block(move || {
        let mut conn = pool_clone.get()?;
        // let hashed_password: String = hash_password(&body.password)?;

        add_user(&role_id, &body, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let token_clone = token.clone();
    role_id = role[0].id.clone();
    web::block(move || {
        let mut conn = state.pool.get()?;

        add_to_session(&mut conn, &id.to_string(), &user.id, &role_id, &token_clone)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(EncodeResponse {
        message: String::from("Authorized"),
        token,
    }))
}

async fn sign_in(
    body: web::Json<EncodeBody>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut rng = rand::thread_rng();
    let id: usize = rng.gen();
    let exp: usize = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.secret.as_str().as_ref()),
    )
    .unwrap();

    let pool_clone = state.pool.clone();
    let body_clone = body.clone();
    web::block(move || {
        let mut conn = pool_clone.get()?;
        check_user(body_clone.email, &mut conn, false)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let pool_clone = state.pool.clone();
    let users = web::block(move || {
        let mut conn = pool_clone.get()?;
        validate_user(&body.email, &body.password, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let token_clone = token.clone();
    web::block(move || {
        let mut conn = state.pool.get()?;

        add_to_session(
            &mut conn,
            &id.to_string(),
            &users[0].id,
            &users[0].role_id,
            &token_clone,
        )
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(EncodeResponse {
        message: String::from("Authorized"),
        token,
    }))
}

#[derive(Serialize, Deserialize)]
struct DecodeResponse {
    message: String,
    id: usize,
}

#[derive(Serialize, Deserialize)]
struct DecodeBody {
    token: String,
}

async fn decode_token(body: web::Json<DecodeBody>, state: web::Data<AppState>) -> HttpResponse {
    let token_result: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(state.secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match token_result {
        Ok(token) => HttpResponse::Ok().json(DecodeResponse {
            message: String::from("Successfully logged in."),
            id: token.claims.id,
        }),
        Err(e) => HttpResponse::Unauthorized().json(Response {
            message: e.to_string(),
        }),
    }
}

async fn protected(auth_token: AuthenticationToken) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Response {
        message: String::from("Authorized"),
    }))
}

fn add_user(
    roles_id: &str,
    body: &web::Json<EncodeBody>,
    conn: &mut PgConnection,
) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let new_user = NewUser {
        id: &Uuid::new_v4().to_string(),
        role_id: roles_id,
        email: &body.email,
        password: &body.password,
    };

    let res = diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)?;
    Ok(res)
}

fn check_user(
    user_email: String,
    conn: &mut PgConnection,
    sign_up: bool,
) -> Result<Vec<User>, DbError> {
    use crate::schema::users::dsl::*;
    let user = users.filter(email.eq(user_email)).load::<User>(conn)?;

    if sign_up {
        if user.is_empty() {
            Ok(user)
        } else {
            Err("Email has already registered".into())
        }
    } else {
        if user.is_empty() {
            Err("Email has not been registered".into())
        } else {
            Ok(user)
        }
    }
}

fn get_admin(conn: &mut PgConnection) -> Result<Vec<Role>, DbError> {
    use crate::schema::roles::dsl::*;
    let role = roles.filter(name.eq("admin")).load::<Role>(conn)?;
    Ok(role)
}

fn add_to_session(
    conn: &mut PgConnection,
    claim_id: &str,
    user: &str,
    role: &str,
    token: &str,
) -> Result<Session, DbError> {
    use crate::schema::session::dsl::*;

    diesel::delete(session.filter(user_id.eq(user)))
        .filter(expires_at.lt(chrono::Local::now().naive_local()))
        .execute(conn)
        .expect("Error deleting old sessions");

    let new_session = NewSession {
        id: claim_id,
        user_id: user,
        role_id: role,
        access_token: token,
        expires_at: chrono::Local::now().naive_local() + Duration::days(1),
    };

    let res = diesel::insert_into(session)
        .values(new_session)
        .get_result(conn)?;
    Ok(res)
}

fn validate_user(
    user_email: &str,
    user_password: &str,
    conn: &mut PgConnection,
) -> Result<Vec<User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users.filter(email.eq(user_email)).load::<User>(conn)?;

    if !user.is_empty() && user_password == user[0].password {
        Ok(user)
    } else {
        Err("Invalid email or password".into())
    }
}
