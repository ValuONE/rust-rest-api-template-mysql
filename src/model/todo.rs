use std::error::Error;

use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::mysql::MySqlRow;

use crate::repository::mysql::AppState;
use crate::utility::error::ApiError;
use crate::utility::response::BoolResponse;

#[derive(Deserialize, Serialize)]
pub struct CreateTodo {
    pub title: String,
    pub is_finished: Option<bool>,
}

impl CreateTodo {
    pub async fn create_todo(&self, app_state: Data<AppState>) -> Result<Todo, Box<dyn Error>> {
        let query = sqlx::query("INSERT INTO `todos` (`title`, `is_finished`) VALUES (?, ?);")
            .bind(self.title.to_string())
            .bind(self.is_finished)
            .execute(&app_state.pool).await?;

        let todo = sqlx::query("SELECT * FROM `todos` WHERE `id` = ?;")
            .bind(query.last_insert_id())
            .map(|row: MySqlRow| Todo::from_row(&row))
            .fetch_one(&app_state.pool).await??;

        Ok(todo)
    }
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub is_finished: bool,
}

impl Todo {
    pub async fn get_todos(app_state: Data<AppState>) -> Result<Vec<Self>, Box<dyn Error>> {
        let todos = sqlx::query_as::<_, Todo>("SELECT * FROM `todos`;")
            .fetch_all(&app_state.pool)
            .await?;

        Ok(todos)
    }

    pub async fn get_todo_by_id(id: i64, app_state: Data<AppState>) -> Result<Self, Box<dyn Error>> {
        let mut todos = sqlx::query_as::<_, Todo>(&*format!("SELECT * FROM `todos` WHERE `id` = {};", id))
            .fetch_all(&app_state.pool)
            .await?;

        match todos.len() {
            0 => Err(Box::new(ApiError::TodoNotFound)),
            _ => Ok(todos.remove(0))
        }
    }

    pub async fn delete_todo(id: i64, app_state: Data<AppState>) -> Result<BoolResponse, Box<dyn Error>> {
        let query = sqlx::query("DELETE FROM `todos` WHERE `id` = ?")
            .bind(id)
            .execute(&app_state.pool)
            .await?;

        match query.rows_affected() {
            0 => Err(Box::new(ApiError::TodoNotFound)),
            1 => Ok(BoolResponse::new(true)),
            _ => Ok(BoolResponse::new(false))
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PatchTodo {
    pub title: Option<String>,
    pub is_finished: Option<bool>,
}

impl PatchTodo {
    pub async fn update_todo(&self, id: i64, app_state: Data<AppState>) -> Result<Todo, Box<dyn Error>> {
        let todo = Todo::get_todo_by_id(id, app_state.clone()).await?;

        sqlx::query("UPDATE `todos` SET `title` = ?, `is_finished` = ? WHERE `id` = ?")
            .bind(self.title.clone().unwrap_or(todo.title))
            .bind(self.is_finished.unwrap_or(todo.is_finished))
            .bind(id)
            .execute(&app_state.pool)
            .await?;

        Ok(Todo::get_todo_by_id(id, app_state.clone()).await?)
    }
}