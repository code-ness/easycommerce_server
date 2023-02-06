use actix_web::http::StatusCode;

pub fn hash_password(password: &str) -> Result<String, StatusCode> {
    bcrypt::hash(password.to_owned(), 14).map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)
}
