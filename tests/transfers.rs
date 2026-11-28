use ledger::accounts::{
    self,
    error::AccountError,
    model::{AccountName, Currency},
};
use ledger::transfers::{
    self,
    error::TransferError,
    model::{NewTransfer, Transfer},
};
use sqlx::PgPool;

async fn settlement(pool: &PgPool, currency: &str) -> i64 {
    sqlx::query_scalar!(
        r#"
            SELECT id FROM accounts WHERE allow_negative AND currency = $1
        "#,
        currency
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn new_account(pool: &PgPool, name: &str, currency: &str) -> i64 {
    let name = AccountName::parse(name).unwrap();
    let currency = Currency::parse(currency).unwrap();
    accounts::db::create(pool, &name, &currency)
        .await
        .unwrap()
        .id
}

async fn send(pool: &PgPool, from: i64, to: i64, amount: i64) -> Result<Transfer, TransferError> {
    let t = NewTransfer::new(from, to, amount).unwrap();
    transfers::db::transfer(pool, &t).await
}

async fn balance(pool: &PgPool, id: i64) -> i64 {
    accounts::db::balance(pool, id).await.unwrap().balance
}

async fn count_total_transactions(pool: &PgPool) -> i64 {
    sqlx::query_scalar!(
        r#"
            SELECT COUNT(*) as "count!"
            FROM transactions
        "#
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn count_transaction_id(pool: &PgPool, t_id: i64) -> i64 {
    sqlx::query_scalar!(
        r#"
            SELECT COUNT(*) as "count!"
            FROM entries
            WHERE transaction_id = $1
        "#,
        t_id
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn sum_transaction_id_amount(pool: &PgPool, t_id: i64) -> i64 {
    sqlx::query_scalar!(
        r#"
            SELECT COALESCE(SUM(amount), 0)::BIGINT AS "total!"
            FROM entries
            WHERE transaction_id = $1
        "#,
        t_id
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn ledger_balance(pool: &PgPool) -> i64 {
    sqlx::query_scalar!(
        r#"
            SELECT COALESCE(SUM(amount), 0)::BIGINT AS "total!"
            FROM entries
        "#,
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

#[sqlx::test]
async fn balances_add_up(pool: PgPool) {
    let gbp = settlement(&pool, "GBP").await;
    let alice = new_account(&pool, "Alice", "GBP").await;
    let bob = new_account(&pool, "Bob", "GBP").await;

    send(&pool, gbp, alice, 10_000).await.unwrap();
    send(&pool, alice, bob, 2_500).await.unwrap();

    assert_eq!(balance(&pool, alice).await, 7_500);
    assert_eq!(balance(&pool, bob).await, 2_500);
    assert_eq!(balance(&pool, gbp).await, -10_000);
}

#[sqlx::test]
async fn transfer_writes_two_balanced_entries(pool: PgPool) {
    let gbp = settlement(&pool, "GBP").await;
    let alice = new_account(&pool, "Alice", "GBP").await;
    let bob = new_account(&pool, "Bob", "GBP").await;

    send(&pool, gbp, alice, 10_000).await.unwrap();
    let t = send(&pool, alice, bob, 2_500).await.unwrap();
    assert_eq!(count_transaction_id(&pool, t.transaction_id).await, 2);
    assert_eq!(sum_transaction_id_amount(&pool, t.transaction_id).await, 0);
}

#[sqlx::test]
async fn insufficient_funds_writes_nothing(pool: PgPool) {
    let alice = new_account(&pool, "Alice", "GBP").await;
    let bob = new_account(&pool, "Bob", "GBP").await;

    let t = send(&pool, alice, bob, 2_500).await;
    assert!(matches!(t, Err(TransferError::InsufficientFunds)));
    assert_eq!(count_total_transactions(&pool).await, 0);
}

#[sqlx::test]
async fn currency_mismatch_is_rejected(pool: PgPool) {
    let alice = new_account(&pool, "Alice", "GBP").await;
    let bob = new_account(&pool, "Bob", "USD").await;

    let t = send(&pool, alice, bob, 2_500).await;
    assert!(matches!(t, Err(TransferError::CurrencyMismatch)));
    assert_eq!(count_total_transactions(&pool).await, 0);
}

#[sqlx::test]
async fn unknown_account_is_rejected(pool: PgPool) {
    let bob = new_account(&pool, "Bob", "USD").await;

    let t = send(&pool, 999_i64, bob, 2_500).await;
    assert!(matches!(t, Err(TransferError::AccountNotFound)));
    assert_eq!(count_total_transactions(&pool).await, 0);
}

#[sqlx::test]
async fn new_account_has_zero_balance(pool: PgPool) {
    let bob = new_account(&pool, "Bob", "USD").await;
    assert_eq!(balance(&pool, bob).await, 0)
}

#[sqlx::test]
async fn balance_of_unknown_account_not_found(pool: PgPool) {
    assert!(matches!(
        accounts::db::balance(&pool, 999).await,
        Err(AccountError::NotFound)
    ));
}

#[sqlx::test]
async fn ledger_always_sum_to_zero(pool: PgPool) {
    let gbp = settlement(&pool, "GBP").await;
    let alice = new_account(&pool, "Alice", "GBP").await;
    let bob = new_account(&pool, "Bob", "GBP").await;

    send(&pool, gbp, alice, 10_000).await.unwrap();
    send(&pool, alice, bob, 2_500).await.unwrap();

    let usd = settlement(&pool, "USD").await;
    let sam = new_account(&pool, "Sam", "USD").await;
    let sarah = new_account(&pool, "Sarah", "USD").await;

    send(&pool, usd, sam, 8_000).await.unwrap();
    send(&pool, sam, sarah, 2_000).await.unwrap();

    assert_eq!(balance(&pool, alice).await, 7_500);
    assert_eq!(balance(&pool, bob).await, 2_500);
    assert_eq!(balance(&pool, gbp).await, -10_000);

    assert_eq!(balance(&pool, sam).await, 6_000);
    assert_eq!(balance(&pool, sarah).await, 2_000);
    assert_eq!(balance(&pool, usd).await, -8_000);

    assert_eq!(ledger_balance(&pool).await, 0)
}
