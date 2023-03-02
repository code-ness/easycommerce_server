use crate::{
    extractors::authentication_token::AuthenticationToken,
    models::{NewStore, NewUserStore, Session, Store, UserStore},
    AppState,
};
use actix_web::{web, Error, HttpResponse, Scope};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn store_scope() -> Scope {
    web::scope("stores")
        .route("", web::get().to(get_stores))
        .route("", web::post().to(create_store))
        .route("/{id}", web::get().to(get_store))
        .route("/{id}", web::put().to(update_store))
        .route("/{id}", web::delete().to(delete_store))
}

#[derive(Debug, Serialize, Deserialize)]
struct StoreJoined {
    id: String,
    user_id: String,
    name: String,
    stage: String,
}

async fn get_stores(
    auth_token: AuthenticationToken,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", auth_token.id);
    let pool = state.pool.clone();
    let sessions = web::block(move || {
        let mut conn = pool.get()?;
        get_session(&auth_token.id.to_string(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let pool = state.pool.clone();
    let stores = web::block(move || {
        let mut conn = pool.get()?;
        get_user_stores(&sessions[0].user_id, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(stores))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StorePayload {
    name: String,
    stage: String,
}

async fn create_store(
    auth_token: AuthenticationToken,
    state: web::Data<AppState>,
    body: web::Json<StorePayload>,
) -> Result<HttpResponse, Error> {
    let pool = state.pool.clone();
    let sessions = web::block(move || {
        let mut conn = pool.get()?;
        get_session(&auth_token.id.to_string(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let pool = state.pool.clone();
    let body_clone = body.clone();
    let store = web::block(move || {
        let mut conn = pool.get()?;
        add_store(&body_clone.name, &body_clone.stage, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    web::block(move || {
        let mut conn = state.pool.get()?;
        add_user_store(&sessions[0].user_id, &store.id, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json("success"))
}

async fn get_store() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("success"))
}

async fn update_store(
    id: web::Path<String>,
    body: web::Json<StorePayload>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let store = web::block(move || {
        let mut conn = state.pool.get()?;
        edit_store(&id, &body.name, &body.stage, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(store))
}

async fn delete_store(
    auth_token: AuthenticationToken,
    id: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let pool = state.pool.clone();
    let sessions = web::block(move || {
        let mut conn = pool.get()?;
        get_session(&auth_token.id.to_string(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let pool = state.pool.clone();
    let id_str = id.clone();
    web::block(move || {
        let mut conn = pool.get()?;
        remove_user_store(&id_str, &sessions[0].user_id, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    web::block(move || {
        let mut conn = state.pool.get()?;
        remove_store(&id, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json("success"))
}

fn get_session(user_session_id: &str, conn: &mut PgConnection) -> Result<Vec<Session>, DbError> {
    use crate::schema::session::dsl::*;

    let sessions = session
        .filter(id.eq(user_session_id))
        .load::<Session>(conn)?;

    if sessions.is_empty() {
        Err("User session not found".into())
    } else {
        Ok(sessions)
    }
}

fn get_user_stores(user: &str, conn: &mut PgConnection) -> Result<Vec<StoreJoined>, DbError> {
    use crate::schema::stores::dsl::*;
    use crate::schema::user_stores::dsl::*;

    let results: Vec<(UserStore, Option<Store>)> = user_stores
        .left_outer_join(stores)
        .filter(store_id.eq(id))
        .load::<(UserStore, Option<Store>)>(conn)?;

    let results = results
        .into_iter()
        .filter_map(|(r#struct, opt)| {
            opt.map(|_store| StoreJoined {
                id: _store.id,
                user_id: r#struct.user_id,
                name: _store.name,
                stage: _store.stage,
            })
        })
        .filter(|_store| _store.user_id == user)
        .collect();

    Ok(results)
}

fn add_store(
    store_name: &str,
    store_stage: &str,
    conn: &mut PgConnection,
) -> Result<Store, DbError> {
    use crate::schema::stores::dsl::*;

    let new_store = NewStore {
        id: &Uuid::new_v4().to_string(),
        name: store_name,
        stage: store_stage,
    };

    let res: Store = diesel::insert_into(stores)
        .values(&new_store)
        .get_result(conn)?;

    Ok(res)
}

fn add_user_store(user: &str, store: &str, conn: &mut PgConnection) -> Result<UserStore, DbError> {
    use crate::schema::user_stores::dsl::*;

    let new_user_store = NewUserStore {
        user_id: user,
        store_id: store,
    };

    let res = diesel::insert_into(user_stores)
        .values(&new_user_store)
        .get_result(conn)?;

    Ok(res)
}

fn edit_store(
    _id: &str,
    _name: &str,
    _stage: &str,
    conn: &mut PgConnection,
) -> Result<Store, DbError> {
    use crate::schema::stores::dsl::*;

    let store = diesel::update(stores.find(_id))
        .set((name.eq(_name), stage.eq(_stage)))
        .get_result::<Store>(conn)?;
    Ok(store)
}

fn remove_user_store(
    _id: &str,
    _user_id: &str,
    conn: &mut PgConnection,
) -> Result<UserStore, DbError> {
    use crate::schema::user_stores::dsl::*;

    let user_store =
        diesel::delete(user_stores.find((_user_id, _id))).get_result::<UserStore>(conn)?;
    Ok(user_store)
}

fn remove_store(_id: &str, conn: &mut PgConnection) -> Result<Store, DbError> {
    use crate::schema::stores::dsl::*;

    let count = diesel::delete(stores.find(_id)).get_result::<Store>(conn)?;

    Ok(count)
}
