use crate::{data::*, error, error::Error::*, DBCon, DBPool};
use chrono::prelude::*;
use mobc::Pool;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls, Row};

type Result<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const INIT_SQL: &str = "./db.sql";
const TABLE: &str = "todo";
const SELECT_FIELDS: &str = "id, name, created_at, checked";

pub async fn init_db(db_pool: &DBPool) -> std::result::Result<(), error::Error> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con = get_db_con(db_pool).await?;
    let _ = con
        .batch_execute(init_file.as_str())
        .await
        .map_err(DBInitError)?;
    Ok(())
}

pub async fn get_db_con(db_pool: &DBPool) -> std::result::Result<DBCon, error::Error> {
    db_pool.clone().get().await.map_err(DBPoolError)
}

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str("postgres://postgres@127.0.0.1:7878/postgres")?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

pub async fn fetch_todos(db_pool: &DBPool, search: Option<String>) -> Result<Vec<Todo>> {
    let con = get_db_con(db_pool).await?;
    let where_clause = match search {
        Some(_) => "WHERE name like '%$1%'",
        None => "",
    };
    let query = format!(
        "SELECT {} FROM {} {} ORDER BY created_at DESC",
        SELECT_FIELDS, TABLE, where_clause
    );
    let q = match search {
        Some(v) => con.query(query.as_str(), &[&v]).await,
        None => con.query(query.as_str(), &[]).await,
    };
    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_todo(&r)).collect())
}

pub async fn create_todo(db_pool: &DBPool, body: TodoRequest) -> Result<Todo> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.name])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_todo(&row))
}

pub async fn update_todo(db_pool: &DBPool, id: i32, body: TodoUpdateRequest) -> Result<Todo> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "UPDATE {} SET name = $1, checked = $2 WHERE id = $3 RETURNING *",
        TABLE
    );
    let row = con
        .query_one(query.as_str(), &[&body.name, &body.checked, &id])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_todo(&row))
}

pub async fn delete_todo(db_pool: &DBPool, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)
}

fn row_to_todo(row: &Row) -> Todo {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    let created_at: DateTime<Utc> = row.get(2);
    let checked: bool = row.get(3);
    Todo {
        id,
        name,
        created_at,
        checked,
    }
}
