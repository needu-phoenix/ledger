use sqlx::PgPool;

pub mod handlers;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}
