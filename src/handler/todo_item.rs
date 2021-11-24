use axum::{
    extract::{Extension, Path},
    Json,
};

use crate::{
    db::todo_item,
    form,
    model::{AppState, TodoItem, TodoItemID},
    Response,
};

use super::{get_client, log_error, HandlerResult};

pub async fn create(
    Extension(state): Extension<AppState>,
    Json(payload): Json<form::CreateTodoItem>,
) -> HandlerResult<TodoItemID> {
    let handler_name = "todo_item_create";
    let client = get_client(&state, handler_name).await?;
    let result = todo_item::create(&client, payload)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn all(
    Extension(state): Extension<AppState>,
    Path(list_id): Path<i32>,
) -> HandlerResult<Vec<TodoItem>> {
    let handler_name = "todo_item_all";
    let client = get_client(&state, handler_name).await?;
    let result = todo_item::all(&client, list_id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn find(
    Extension(state): Extension<AppState>,
    Path((list_id, item_id)): Path<(i32, i32)>,
) -> HandlerResult<TodoItem> {
    let handler_name = "todo_item_find";
    let client = get_client(&state, handler_name).await?;
    let result = todo_item::find(&client, list_id, item_id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn check(
    Extension(state): Extension<AppState>,
    Path((list_id, item_id)): Path<(i32, i32)>,
) -> HandlerResult<bool> {
    let handler_name = "todo_item_check";
    let client = get_client(&state, handler_name).await?;
    let result = todo_item::check(&client, list_id, item_id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}
pub async fn delete(
    Extension(state): Extension<AppState>,
    Path((list_id, item_id)): Path<(i32, i32)>,
) -> HandlerResult<bool> {
    let handler_name = "todo_item_delete";
    let client = get_client(&state, handler_name).await?;
    let result = todo_item::delete(&client, list_id, item_id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}
