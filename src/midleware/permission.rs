use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Admin,
    User,
    Student,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Permission {
    Read,
    Write,
    Delete,
}

impl Role {
    pub fn from_str(role: &str) -> Option<Role> {
        match role {
            "Admin" => Some(Role::Admin),
            "User" => Some(Role::User),
            "Guest" => Some(Role::Student),
            _ => None, // Unknown role
        }
    }
}


//get role permission
fn get_role_permission() -> HashMap<Role, Vec<Permission>> {
    let mut role_permission = HashMap::new();

    role_permission.insert(Role::Admin, vec![Permission::Write, Permission::Read, Permission::Delete]);
    role_permission.insert(Role::User, vec![Permission::Write, Permission::Read]);
    role_permission.insert(Role::Student, vec![Permission::Read]);

    role_permission
}


pub fn has_permission(role: &Role, permission: &Permission) -> bool {
    let role_permission = get_role_permission();
    if let Some(permissions) = role_permission.get(role) {
        permissions.contains(&permission)
    } else {
        false
    }
}