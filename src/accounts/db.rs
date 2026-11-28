use super::{
    error::AccountError,
    model::{Account, AccountName, Balance, Currency},
};
use sqlx::PgPool;

pub async fn create(
    pool: &PgPool,
    name: &AccountName,
    currency: &Currency,
) -> Result<Account, AccountError> {
    let account = sqlx::query_as!(
        Account,
        r#"
        INSERT INTO accounts (name, currency)
        VALUES ($1, $2)
        RETURNING id, name, currency, allow_negative, created_at
        "#,
        name.as_str(),
        currency.as_str()
    )
    .fetch_one(pool)
    .await?;

    Ok(account)
}

pub async fn balance(pool: &PgPool, id: i64) -> Result<Balance, AccountError> {
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
    .ok_or(AccountError::NotFound)?;

    Ok(Balance {
        account_id: balance.account_id,
        balance: balance.balance,
        currency: balance.currency,
    })
}
