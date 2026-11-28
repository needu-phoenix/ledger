use super::error::TransferError;
use chrono::{DateTime, Utc};
use serde::Serialize;

pub struct Money(i64);
pub struct NewTransfer {
    from: i64,
    to: i64,
    amount: Money,
}

#[derive(Serialize)]
pub struct Transfer {
    pub transaction_id: i64,
    pub from_account_id: i64,
    pub to_account_id: i64,
    pub amount: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

impl Money {
    pub fn value(&self) -> i64 {
        self.0
    }

    pub fn parse(amount: i64) -> Result<Self, TransferError> {
        if amount <= 0 {
            return Err(TransferError::InvalidAmount);
        }

        Ok(Money(amount))
    }
}

impl NewTransfer {
    pub fn new(from: i64, to: i64, amount: i64) -> Result<Self, TransferError> {
        let amount = Money::parse(amount)?;

        if from == to {
            return Err(TransferError::SameAccount);
        }

        Ok(NewTransfer { from, to, amount })
    }

    pub fn from(&self) -> i64 {
        self.from
    }

    pub fn to(&self) -> i64 {
        self.to
    }

    pub fn amount(&self) -> i64 {
        self.amount.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_non_positive_amounts() {
        assert!(matches!(Money::parse(0), Err(TransferError::InvalidAmount)));
        assert!(matches!(
            Money::parse(-5),
            Err(TransferError::InvalidAmount)
        ));
    }

    #[test]
    fn accepts_positive_amount() {
        assert!(matches!(Money::parse(1), Ok(Money(1))));
        assert_eq!(Money::parse(1).unwrap().value(), 1);
    }

    #[test]
    fn rejects_same_account() {
        assert!(matches!(
            NewTransfer::new(3, 3, 100),
            Err(TransferError::SameAccount)
        ))
    }
}
