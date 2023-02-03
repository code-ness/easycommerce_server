CREATE TABLE role_permissions (
  role_id uuid, 
  permission_id uuid, 
  FOREIGN KEY (role_id) REFERENCES roles (id), 
  FOREIGN KEY (permission_id) REFERENCES permissions (id)
)