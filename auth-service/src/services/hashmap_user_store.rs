use std::collections::HashMap;

use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        match self.users.get(user.email()) {
            Some(_) => Err(UserStoreError::UserAlreadyExists),
            None => {
                self.users.insert(user.email().to_string(), user);
                Ok(())
            }
        }
    }

    // Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    // Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password() == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let user = User::new("test@example.com".to_string(), "password".to_string(), false);
        let mut store = HashmapUserStore::default();
        let result = store.add_user(user.clone());
        assert!(result.is_ok());
        let result = store.add_user(user);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), UserStoreError::UserAlreadyExists);
    }

    #[tokio::test]
    async fn test_get_user() {
        let user = User::new("test@example.com".to_string(), "password".to_string(), false);
        let mut store = HashmapUserStore::default();
        let result = store.add_user(user.clone());
        assert!(result.is_ok());
        let result = store.get_user("test@example.com");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().email(), "test@example.com");
        let result = store.get_user("nonexistent@example.com");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), UserStoreError::UserNotFound);
    }

    #[tokio::test]
    async fn test_validate_user() {
         let user = User::new("test@example.com".to_string(), "password".to_string(), false);
        let mut store = HashmapUserStore::default();
        let result = store.add_user(user.clone());
        assert!(result.is_ok());
        let result = store.validate_user("test@example.com", "password");
        assert!(result.is_ok());
        let result = store.validate_user("test@example.com", "wrong_password");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), UserStoreError::InvalidCredentials);
        let result = store.validate_user("nonexistent@example.com", "password");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), UserStoreError::UserNotFound);
    }
}