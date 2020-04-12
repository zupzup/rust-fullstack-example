use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub checked: bool,
}

#[derive(Deserialize)]
pub struct TodoRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct TodoUpdateRequest {
    pub name: String,
    pub checked: bool,
}

#[derive(Serialize)]
pub struct TodoResponse {
    pub id: i32,
    pub name: String,
    pub checked: bool,
}

impl TodoResponse {
    pub fn of(todo: Todo) -> TodoResponse {
        TodoResponse {
            id: todo.id,
            name: todo.name,
            checked: todo.checked,
        }
    }
}
