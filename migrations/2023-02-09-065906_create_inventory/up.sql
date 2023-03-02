CREATE TABLE inventory (
  user_id VARCHAR REFERENCES users (id) NOT NULL,
  product_id VARCHAR REFERENCES products (id) NOT NULL,
  PRIMARY KEY (user_id, product_id)
)