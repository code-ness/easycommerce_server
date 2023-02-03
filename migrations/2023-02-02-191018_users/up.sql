CREATE TABLE users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
  role_id uuid,
  email VARCHAR,
  password VARCHAR,
  FOREIGN KEY (role_id) REFERENCES roles (id)
)