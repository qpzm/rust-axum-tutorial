use axum::extract::FromRef;
use axum_macros::FromRef;
use axum_example_service::sea_orm::DatabaseConnection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
}
