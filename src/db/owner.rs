use super::{get_db_con, Result};
use crate::{data::*, error::Error::*, DBPool};
use mobc_postgres::tokio_postgres::Row;

pub const TABLE: &str = "owner";
const SELECT_FIELDS: &str = "id, name";

pub async fn fetch(db_pool: &DBPool) -> Result<Vec<Owner>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {}", SELECT_FIELDS, TABLE);
    let rows = con.query(query.as_str(), &[]).await.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_owner(&r)).collect())
}

pub async fn create(db_pool: &DBPool, body: OwnerRequest) -> Result<Owner> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.name])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_owner(&row))
}

pub async fn update(db_pool: &DBPool, id: i32, body: OwnerUpdateRequest) -> Result<Owner> {
    let con = get_db_con(db_pool).await?;
    let query = format!("UPDATE {} SET name = $1 WHERE id = $3 RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.name, &id])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_owner(&row))
}

pub async fn delete(db_pool: &DBPool, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)
}

fn row_to_owner(row: &Row) -> Owner {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    Owner { id, name }
}
