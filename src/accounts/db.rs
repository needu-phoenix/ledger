use super::model::Account;
use sqlx::{self, postgres::PgPool};

pub async fn create(pool: &PgPool, name: &str, currency: &str) -> Result<Account, sqlx::Error> {
    let account = sqlx::query_as!(
        Account,
        r#"
        INSERT INTO accounts (name, currency)
        VALUES ($1, $2)
        RETURNING *
        "#,
        name,
        currency
    )
    .fetch_one(pool)
    .await?;

    Ok(account)
}
