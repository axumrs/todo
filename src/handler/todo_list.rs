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

use super::{get_client, log_error};

pub async fn create(
    Extension(state): Extension<AppState>,
    Json(payload): Json<form::CreateTodoList>,
) -> Result<Json<Response<TodoListID>>> {
    let handler_name = "todo_list_create";
    let client = get_client(&state, handler_name).await?;
    let result = todo_list::create(&client, payload)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn all(Extension(state): Extension<AppState>) -> Result<Json<Response<Vec<TodoList>>>> {
    let handler_name = "todo_list_all";
    let client = get_client(&state, handler_name).await?;
    let result = todo_list::all(&client)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn find(
    Extension(state): Extension<AppState>,
    Path(list_id): Path<i32>,
) -> Result<Json<Response<TodoList>>> {
    let handler_name = "todo_list_find";
    let client = get_client(&state, handler_name).await?;
    let result = todo_list::find(&client, list_id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}
pub async fn delete(
    Extension(state): Extension<AppState>,
    Path(list_id): Path<i32>,
) -> Result<Json<Response<bool>>> {
    let handler_name = "todo_list_delete";
    let mut client = get_client(&state, handler_name).await?;
    let result = todo_list::delete(&mut client, list_id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}
pub async fn update(
    Extension(state): Extension<AppState>,
    Json(payload): Json<form::UpdateTodoList>,
) -> Result<Json<Response<bool>>> {
    let handler_name = "todo_list_update";
    let client = get_client(&state, handler_name).await?;
    let result = todo_list::update(&client, payload)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}
