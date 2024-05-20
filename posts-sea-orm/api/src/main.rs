mod app_state;
mod queries;

use std::env;
use std::time::Duration;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use serde::{Deserialize, Serialize};
use tracing::info;
use axum_example_service::{
    sea_orm::{Database, DatabaseConnection},
};
use axum_example_service::sea_orm::ActiveModelTrait;
use axum_example_service::sea_orm::ActiveValue::Set;
use crate::app_state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = match Database::connect(db_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    let app_state = AppState {
        db,
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/posts", get(list_tasks).post(create_task))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct ResponsePost {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataPost {
    pub data: ResponsePost,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataPosts {
    pub data: Vec<ResponsePost>,
}

async fn list_tasks(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    let tasks = get_all_tasks(&db).await.unwrap();
    info!("tasks: {:?}", tasks);

    StatusCode::OK
}

/*
pub async fn get_all_posts(
    // Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataPosts>, _> {
    let tasks = task_queries::get_all_tasks(&db, user.id, false)
        .await?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            description: db_task.description,
            priority: db_task.priority,
            completed_at: db_task
                .completed_at
                .map(|completed_at| completed_at.to_string()),
        })
        .collect::<Vec<ResponseTask>>();

    Ok(Json(ResponseDataTasks { data: tasks }))
}
 */
