use super::{
    error::TransferError,
    model::{NewTransfer, Transfer},
};
use crate::transfers::model::Balance;
use sqlx::PgPool;

pub async fn transfer(pool: &PgPool, t: &NewTransfer) -> Result<Transfer, TransferError> {
    let mut tx = pool.begin().await?;

    let accounts = sqlx::query!(
        r#"
            SELECT id, currency, allow_negative 
            FROM accounts
            WHERE id IN ($1, $2)
        "#,
        t.from(),
        t.to()
    )
    .fetch_all(&mut *tx)
    .await?;

    if accounts.len() != 2 {
        return Err(TransferError::AccountNotFound);
    }

    if accounts[0].currency != accounts[1].currency {
        return Err(TransferError::CurrencyMismatch);
    }

    let sender = accounts
        .iter()
        .find(|a| a.id == t.from())
        .ok_or(TransferError::AccountNotFound)?;

    let sender_balance = sqlx::query!(
        r#"
            SELECT COALESCE(SUM(amount), 0)::BIGINT AS "balance!"
            FROM entries
            WHERE account_id = $1
        "#,
        t.from()
    )
    .fetch_one(&mut *tx)
    .await?;

    if !sender.allow_negative && sender_balance.balance < t.amount() {
        return Err(TransferError::InsufficientFunds);
    }

    let transaction = sqlx::query!(
        r#"
            INSERT INTO transactions (kind)
            VALUES ('transfer')
            RETURNING id, created_at
        "#
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query!(
        r#"
            INSERT INTO entries (transaction_id, account_id, amount)
            VALUES
                ($1, $2, $3),
                ($1, $4, $5)
        "#,
        transaction.id,
        t.from(),
        -t.amount(),
        t.to(),
        t.amount()
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Transfer {
        transaction_id: transaction.id,
        from_account_id: t.from(),
        to_account_id: t.to(),
        amount: t.amount(),
        currency: sender.currency.clone(),
        created_at: transaction.created_at,
    })
}

pub async fn balance(pool: &PgPool, id: i64) -> Result<Balance, TransferError> {
    let balance = sqlx::query!(
        r#"
            SELECT a.id AS account_id, a.currency, COALESCE(SUM(e.amount), 0)::BIGINT AS "balance!"
            FROM accounts a 
            LEFT JOIN entries e 
            ON e.account_id = a.id
            WHERE a.id = $1
            GROUP BY a.id, a.currency
        "#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(TransferError::AccountNotFound)?;

    Ok(Balance {
        account_id: balance.account_id,
        balance: balance.balance,
        currency: balance.currency,
    })
}
