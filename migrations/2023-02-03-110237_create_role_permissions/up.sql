CREATE TABLE role_permissions (
  role_id VARCHAR REFERENCES roles (id) NOT NULL,
  permission_id VARCHAR REFERENCES permissions (id) NOT NULL,
  PRIMARY KEY (role_id, permission_id)
)