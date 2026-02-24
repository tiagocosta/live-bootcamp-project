#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) ->  Result<Self, String> {
        if password.trim().len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }
        Ok(Password(password))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let password = Password::parse("strongpassword".to_string());
        assert!(password.is_ok());
    }

    #[test]
    fn test_invalid_password() {
        let password = Password::parse("short".to_string());
        assert!(password.is_err());
    }
}