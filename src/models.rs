use crate::schema::{roles, session, users};
use diesel::{prelude::*, sql_types::Date};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Role {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole<'a> {
    pub name: &'a str,
}
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: String,
    pub role_id: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub role_id: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub access_token: String,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = session)]
pub struct NewSession<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub role_id: &'a str,
    pub access_token: &'a str,
    pub expires_at: chrono::NaiveDateTime,
}
