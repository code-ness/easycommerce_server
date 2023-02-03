CREATE TABLE permissions (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
  name VARCHAR NOT NULL
)