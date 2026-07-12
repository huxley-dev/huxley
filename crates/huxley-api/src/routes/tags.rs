use axum::{
    extract::{Path, State},
    routing::{get, post, patch, delete},
    Json,
    Router,
};
use uuid::Uuid;

use huxley_state::HuxleyState;
use huxley_store::commands::{CreateTag, UpdateTag};
use crate::{
    dtos::tag::{TagResponse, CreateTagRequest, UpdateTagRequest},
    HuxleyApiResult,
};

pub fn router() -> Router<HuxleyState> {
    Router::new()
        .route("/", get(list_handler).post(create_handler))
        .route("/{id}", get(get_handler).patch(update_handler).delete(delete_handler))
}

async fn get_handler(State(state): State<HuxleyState>, Path(id): Path<Uuid>) -> HuxleyApiResult<Json<TagResponse>> {
    let tag = state.tags_repo.find_by_id(state.db_pool, id)
        .await?
        .ok_or()?;

    Ok(Json(tag.into()))
}

async fn list_handler(State(state): State<HuxleyState>) -> HuxleyApiResult<> {

}

async fn create_handler(State(state): State<HuxleyState>, Json(req): Json<CreateTagRequest>) -> HuxleyApiResult<Json<TagResponse>> {
    let cmd = CreateTag::try_from(req)?;
    let tag = state.tags_repo.create(conn, cmd).await?;

    Ok(Json(tag.into()))
}

async fn update_handler(State(state): State<HuxleyState>, Path(id): Path<Uuid>, Json(req): Json<UpdateTagRequest>) -> HuxleyApiResult<Json<TagResponse>> {
    let cmd = UpdateTag::from(req)?;
    let tag = state.tags_repo.update(conn, id, cmd).await?;

    Ok(Json(tag.into()))
}

async fn delete_handler(State(state): State<HuxleyState>, Path(id): Path<Uuid>) -> HuxleyApiResult<> {

}
