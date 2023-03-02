use crate::schema::{inventory, products, roles, session, stores, user_stores, users};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Role {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole<'a> {
    pub id: &'a str,
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

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Store {
    pub id: String,
    pub name: String,
    pub stage: String,
}

#[derive(Insertable)]
#[diesel(table_name = stores)]
pub struct NewStore<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub stage: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserStore {
    pub user_id: String,
    pub store_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = user_stores)]
pub struct NewUserStore<'a> {
    pub user_id: &'a str,
    pub store_id: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub price: &'a f64,
    pub quantity: &'a i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Inventory {
    pub user_id: String,
    pub product_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = inventory)]
pub struct NewInventory<'a> {
    pub user_id: &'a str,
    pub product_id: &'a str,
}
