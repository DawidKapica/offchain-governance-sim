use serde::{Deserialize, Serialize};

/// Different roles have different vote weights.
#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    User,       // weight 1
    Validator,  // weight 3
    Admin,      // weight 5
}

impl Role {
    pub fn weight(&self) -> u32 {
        match self {
            Role::User => 1,
            Role::Validator => 3,
            Role::Admin => 5,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub role: Role,
}

impl User {
    pub fn new(id: u32, name: String, role: Role) -> Self {
        User { id, name, role }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_weight() {
        assert_eq!(Role::User.weight(), 1);
        assert_eq!(Role::Validator.weight(), 3);
        assert_eq!(Role::Admin.weight(), 5);
    }

    #[test]
    fn test_user_creation() {
        let user = User::new(42, "Alice".to_string(), Role::Admin);
        assert_eq!(user.id, 42);
        assert_eq!(user.name, "Alice");
        assert!(matches!(user.role, Role::Admin));
    }
}
