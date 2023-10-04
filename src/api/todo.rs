use actix_web::{delete, get, HttpResponse, post, put};
use actix_web::web::{Data, Json, Path};

use crate::model::todo::{CreateTodo, PatchTodo, Todo};
use crate::repository::mysql::AppState;
use crate::utility::response::generate_response;

#[get("/todo")]
pub async fn get_all_todos(app_state: Data<AppState>) -> HttpResponse {
    generate_response(Todo::get_todos(app_state).await)
}

#[post("/todo")]
pub async fn create_todo(create: Json<CreateTodo>, app_state: Data<AppState>) -> HttpResponse {
    generate_response(create.create_todo(app_state).await)
}

#[get("/todo/{id}")]
pub async fn get_todo_by_id(id: Path<i64>, app_state: Data<AppState>) -> HttpResponse {
    generate_response(Todo::get_todo_by_id(id.into_inner(), app_state).await)
}

#[put("/todo/{id}")]
pub async fn update_todo(id: Path<i64>, patch: Json<PatchTodo>, app_state: Data<AppState>) -> HttpResponse {
    generate_response(patch.update_todo(id.into_inner(), app_state).await)
}

#[delete("/todo/{id}")]
pub async fn delete_todo(id: Path<i64>, app_state: Data<AppState>) -> HttpResponse {
    generate_response(Todo::delete_todo(id.into_inner(), app_state).await)
}
