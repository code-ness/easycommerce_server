// @generated automatically by Diesel CLI.

diesel::table! {
    inventory (user_id, product_id) {
        user_id -> Varchar,
        product_id -> Varchar,
    }
}

diesel::table! {
    permissions (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

diesel::table! {
    products (id) {
        id -> Varchar,
        title -> Varchar,
        description -> Nullable<Varchar>,
        price -> Float8,
        quantity -> Int4,
    }
}

diesel::table! {
    role_permissions (role_id, permission_id) {
        role_id -> Varchar,
        permission_id -> Varchar,
    }
}

diesel::table! {
    roles (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

diesel::table! {
    session (id) {
        id -> Varchar,
        user_id -> Varchar,
        role_id -> Varchar,
        access_token -> Varchar,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    stores (id) {
        id -> Varchar,
        name -> Varchar,
        stage -> Varchar,
    }
}

diesel::table! {
    user_stores (user_id, store_id) {
        user_id -> Varchar,
        store_id -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        role_id -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(inventory -> products (product_id));
diesel::joinable!(inventory -> users (user_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(session -> roles (role_id));
diesel::joinable!(session -> users (user_id));
diesel::joinable!(user_stores -> stores (store_id));
diesel::joinable!(user_stores -> users (user_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    inventory,
    permissions,
    products,
    role_permissions,
    roles,
    session,
    stores,
    user_stores,
    users,
);
