use super::error::AccountError;
use chrono::{DateTime, Utc};
use serde::Serialize;

pub const MAX_NAME_LEN: usize = 100;

#[derive(Serialize, Debug)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub currency: String,
    pub allow_negative: bool,
    pub created_at: DateTime<Utc>,
}

pub struct AccountName(String);
pub struct Currency(String);

impl AccountName {
    pub fn parse(s: &str) -> Result<Self, AccountError> {
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err(AccountError::EmptyName);
        }

        if trimmed.chars().count() > MAX_NAME_LEN {
            return Err(AccountError::NameTooLong);
        }

        Ok(AccountName(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Currency {
    pub fn parse(s: &str) -> Result<Self, AccountError> {
        if s.len() == 3 && s.bytes().all(|b| b.is_ascii_alphabetic()) {
            Ok(Currency(s.to_ascii_uppercase()))
        } else {
            Err(AccountError::InvalidCurrency(s.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for Currency {
    fn default() -> Self {
        Currency("GBP".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_empty_name() {
        for item in ["", " "] {
            assert!(matches!(
                AccountName::parse(item),
                Err(AccountError::EmptyName)
            ))
        }
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let name = AccountName::parse(" Alice ").unwrap();
        assert!(matches!(name.as_str(), "Alice"))
    }

    #[test]
    fn reject_invalid_currencies() {
        for bad in ["G1P", "A B", "GBPP", "AB", "$$$", ""] {
            assert!(
                matches!(Currency::parse(bad), Err(AccountError::InvalidCurrency(_))),
                "expected {bad:?} to be rejected"
            );
        }
    }
}
