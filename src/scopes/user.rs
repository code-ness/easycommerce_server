use crate::{
    extractors::authentication_token::{AuthenticationToken, Claims},
    AppState,
};
use actix_web::{web, HttpResponse, Scope};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/encode-token/{id}", web::get().to(encode_token))
        .route("/decode-token", web::post().to(decode_token))
        .route("/login", web::get().to(login))
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct EncodeBody {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct EncodeResponse {
    message: String,
    token: String,
}

async fn encode_token(
    path: web::Path<usize>,
    body: web::Json<EncodeBody>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let id: usize = path.into_inner();
    let exp: usize = (Utc::now() + Duration::hours(1)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.secret.as_str().as_ref()),
    )
    .unwrap();
    println!("encode {}", state.secret);
    HttpResponse::Ok().json(EncodeResponse {
        message: String::from("success"),
        token,
    })
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

async fn login(auth_token: AuthenticationToken) -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: String::from("Authorized"),
    })
}
