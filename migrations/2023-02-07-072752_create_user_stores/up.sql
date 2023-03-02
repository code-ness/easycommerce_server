CREATE TABLE user_stores (
  user_id VARCHAR REFERENCES users (id) NOT NULL,
  store_id VARCHAR REFERENCES stores (id) NOT NULL,
  PRIMARY KEY (user_id, store_id)
)