#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Self, String> {
        if email.contains('@') {
            Ok(Email(email))
        } else {
            Err("Invalid email format".to_string())
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::parse("test@example.com".to_string());
        assert!(email.is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let email = Email::parse("invalid_email".to_string());
        assert!(email.is_err());
    }
}