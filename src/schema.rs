// @generated automatically by Diesel CLI.

diesel::table! {
    permissions (id) {
        id -> Varchar,
        name -> Varchar,
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
    users (id) {
        id -> Varchar,
        role_id -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(session -> roles (role_id));
diesel::joinable!(session -> users (user_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    permissions,
    role_permissions,
    roles,
    session,
    users,
);
