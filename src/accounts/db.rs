use super::model::{Account, AccountName, Currency};
use sqlx::PgPool;

pub async fn create(
    pool: &PgPool,
    name: &AccountName,
    currency: &Currency,
) -> Result<Account, sqlx::Error> {
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
