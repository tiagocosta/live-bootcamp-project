use std::collections::HashMap;

use crate::domain::{data_stores::{UserStore, UserStoreError}, email::Email, password::Password, user::User};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        match self.users.get(user.email()) {
            Some(_) => Err(UserStoreError::UserAlreadyExists),
            None => {
                self.users.insert(user.email().clone(), user);
                Ok(())
            }
        }
    }

    // Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    async fn get_user(&self, email: &Email) -> Result<&User, UserStoreError> {
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
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
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

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let user = User::new(Email::parse("test@example.com".to_string()).unwrap(), Password::parse("password".to_string()).unwrap(), false);
        let mut store = HashmapUserStore::default();
        let result = store.add_user(user.clone()).await;
        assert!(result.is_ok());
        let result = store.add_user(user).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), UserStoreError::UserAlreadyExists);
    }

    #[tokio::test]
    async fn test_get_user() {
        let user = User::new(Email::parse("test@example.com".to_string()).unwrap(), Password::parse("password".to_string()).unwrap(), false);
        let mut store = HashmapUserStore::default();
        let result = store.add_user(user.clone()).await;
        assert!(result.is_ok());
        let result = store.get_user(&Email::parse("test@example.com".to_string()).unwrap()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().email(), &Email::parse("test@example.com".to_string()).unwrap());
        let result = store.get_user(&Email::parse("nonexistent@example.com".to_string()).unwrap()).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), UserStoreError::UserNotFound);
    }

    #[tokio::test]
    async fn test_validate_user() {
         let user = User::new(Email::parse("test@example.com".to_string()).unwrap(), Password::parse("password".to_string()).unwrap(), false);
        let mut store = HashmapUserStore::default();
        let result = store.add_user(user.clone()).await;
        assert!(result.is_ok());
        let result = store.validate_user(&Email::parse("test@example.com".to_string()).unwrap(), &Password::parse("password".to_string()).unwrap()).await;
        assert!(result.is_ok());
        let result = store.validate_user(&Email::parse("test@example.com".to_string()).unwrap(), &Password::parse("wrong_password".to_string()).unwrap()).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), UserStoreError::InvalidCredentials);
        let result = store.validate_user(&Email::parse("nonexistent@example.com".to_string()).unwrap(), &Password::parse("password".to_string()).unwrap()).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), UserStoreError::UserNotFound);
    }
}