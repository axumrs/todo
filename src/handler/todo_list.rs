use axum::{
    extract::{Extension, Path},
    Json,
};

use crate::{
    db::todo_list,
    error::AppError,
    form,
    model::{AppState, TodoList, TodoListID},
    Response, Result,
};

pub async fn create(
    Extension(state): Extension<AppState>,
    Json(payload): Json<form::CreateTodoList>,
) -> Result<Json<Response<TodoListID>>> {
    let client = state.pool.get().await.map_err(AppError::from)?;
    let result = todo_list::create(&client, payload).await?;
    Ok(Json(Response::ok(result)))
}

pub async fn all(Extension(state): Extension<AppState>) -> Result<Json<Response<Vec<TodoList>>>> {
    let client = state.pool.get().await.map_err(AppError::from)?;
    let result = todo_list::all(&client).await?;
    Ok(Json(Response::ok(result)))
}

pub async fn find(
    Extension(state): Extension<AppState>,
    Path(list_id): Path<i32>,
) -> Result<Json<Response<TodoList>>> {
    let client = state.pool.get().await.map_err(AppError::from)?;
    let result = todo_list::find(&client, list_id).await?;
    Ok(Json(Response::ok(result)))
}
pub async fn delete(
    Extension(state): Extension<AppState>,
    Path(list_id): Path<i32>,
) -> Result<Json<Response<bool>>> {
    let mut client = state.pool.get().await.map_err(AppError::from)?;
    let result = todo_list::delete(&mut client, list_id).await?;
    Ok(Json(Response::ok(result)))
}
pub async fn update(
    Extension(state): Extension<AppState>,
    Json(payload): Json<form::UpdateTodoList>,
) -> Result<Json<Response<bool>>> {
    let client = state.pool.get().await.map_err(AppError::from)?;
    let result = todo_list::update(&client, payload).await?;
    Ok(Json(Response::ok(result)))
}
