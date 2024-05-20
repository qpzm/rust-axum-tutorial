use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_example_service::sea_orm::ActiveValue::Set;
use axum_example_service::sea_orm::DatabaseConnection;
use entity::tasks;
use entity::tasks::{get_all_tasks};

async fn create_task(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    let task = task::ActiveModel{
        title: Set("Hello, World!".to_owned()),
        ..Default::default()
    };

    task.insert(&db).await.unwrap();

    StatusCode::ACCEPTED
}

pub async fn get_all_tasks(
    db: &DatabaseConnection,
    user_id: i32,
    get_deleted: bool,
) -> Result<Vec<TaskModel>, AppError> {
    let mut query = Tasks::find().filter(tasks::Column::UserId.eq(Some(user_id)));

    if !get_deleted {
        query = query.filter(tasks::Column::DeletedAt.is_null());
    }

    query.all(db).await.map_err(|error| {
        eprintln!("Error getting all tasks: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all tasks")
    })
}
