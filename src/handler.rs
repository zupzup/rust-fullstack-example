use crate::{data::*, db, error::Error::*, DBPool, Result};
use serde_derive::Deserialize;
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Deserialize)]
pub struct SearchQuery {
    search: Option<String>,
}

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply> {
    let db = db::get_db_con(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;
    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

pub async fn list_todos_handler(query: SearchQuery, db_pool: DBPool) -> Result<impl Reply> {
    let todos = db::fetch_todos(&db_pool, query.search)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &todos.into_iter().map(|t| TodoResponse::of(t)).collect(),
    ))
}

pub async fn create_todo_handler(body: TodoRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&TodoResponse::of(
        db::create_todo(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn update_todo_handler(
    id: i32,
    body: TodoUpdateRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&TodoResponse::of(
        db::update_todo(&db_pool, id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_todo_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    &db::delete_todo(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}
