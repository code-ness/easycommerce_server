CREATE TABLE session (
  id VARCHAR PRIMARY KEY,
  user_id VARCHAR NOT NULL,
  role_id VARCHAR NOT NULL,
  access_token VARCHAR NOT NULL,
  expires_at TIMESTAMP NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (id),
  FOREIGN KEY (role_id) REFERENCES roles (id)
)