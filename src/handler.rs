use crate::{data::*, db, DBPool, Result};
use warp::{http::StatusCode, reject, reply::json, Reply};

pub async fn list_pets_handler(owner_id: i32, db_pool: DBPool) -> Result<impl Reply> {
    let pets = db::pet::fetch(&db_pool, owner_id)
        .await
        .map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &pets.into_iter().map(PetResponse::of).collect(),
    ))
}

pub async fn create_pet_handler(
    owner_id: i32,
    body: PetRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&PetResponse::of(
        db::pet::create(&db_pool, owner_id, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn update_pet_handler(
    owner_id: i32,
    id: i32,
    body: PetUpdateRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&PetResponse::of(
        db::pet::update(&db_pool, owner_id, id, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn delete_pet_handler(owner_id: i32, id: i32, db_pool: DBPool) -> Result<impl Reply> {
    db::pet::delete(&db_pool, owner_id, id)
        .await
        .map_err(reject::custom)?;
    Ok(StatusCode::OK)
}

pub async fn list_owners_handler(db_pool: DBPool) -> Result<impl Reply> {
    let pets = db::owner::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &pets.into_iter().map(OwnerResponse::of).collect(),
    ))
}

pub async fn create_owner_handler(body: OwnerRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&OwnerResponse::of(
        db::owner::create(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn update_owner_handler(
    id: i32,
    body: OwnerUpdateRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&OwnerResponse::of(
        db::owner::update(&db_pool, id, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn delete_owner_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    db::owner::delete(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(StatusCode::OK)
}
