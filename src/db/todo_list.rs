use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    error::AppError,
    form,
    model::{TodoList, TodoListID},
    Result,
};

/// 创建待办列表
pub async fn create(client: &Client, frm: form::CreateTodoList) -> Result<TodoListID> {
    let stmt = client
        .prepare("INSERT INTO todo_list (title) VALUES ($1) RETURNING id")
        .await
        .map_err(AppError::from)?;
    let result = client
        .query(&stmt, &[&frm.title])
        .await
        .map_err(AppError::from)?
        .iter()
        .map(|row| TodoListID::from_row_ref(row).unwrap())
        .collect::<Vec<TodoListID>>()
        .pop()
        .ok_or(AppError::not_found())?;
    Ok(result)
}

/// 所有待办列表
pub async fn all(client: &Client) -> Result<Vec<TodoList>> {
    let stmt = client
        .prepare("SELECT id,title FROM todo_list ORDER BY id DESC")
        .await
        .map_err(AppError::from)?;
    let result = client
        .query(&stmt, &[])
        .await
        .map_err(AppError::from)?
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    Ok(result)
}

/// 根据ID查找待办列表
pub async fn find(client: &Client, list_id: i32) -> Result<TodoList> {
    let stmt = client
        .prepare("SELECT id,title FROM todo_list WHERE id=$1")
        .await
        .map_err(AppError::from)?;
    let result = client
        .query(&stmt, &[&list_id])
        .await
        .map_err(AppError::from)?
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(AppError::not_found())?;
    Ok(result)
}

/// 更新待办列表
pub async fn update(client: &Client, frm: form::UpdateTodoList) -> Result<bool> {
    let stmt = client
        .prepare("UPDATE todo_list SET title=$1 WHERE id=$2")
        .await
        .map_err(AppError::from)?;
    let result = client
        .execute(&stmt, &[&frm.title, &frm.id])
        .await
        .map_err(AppError::from)?;
    Ok(result > 0)
}

/// 删除待办列表
pub async fn delete(client: &mut Client, id: i32) -> Result<bool> {
    let tx = client.transaction().await.map_err(AppError::from)?;
    let stmt = tx
        .prepare("DELETE FROM todo_list  WHERE id=$1")
        .await
        .map_err(AppError::from)?;
    let result = tx.execute(&stmt, &[&id]).await;
    if let Err(err) = result {
        tx.rollback().await.map_err(AppError::from)?;
        return Err(AppError::db_error(err));
    };
    let stmt = tx
        .prepare("DELETE FROM todo_item WHERE list_id=$1")
        .await
        .map_err(AppError::from)?;
    let result = tx.execute(&stmt, &[&id]).await;
    if let Err(err) = result {
        tx.rollback().await.map_err(AppError::from)?;
        return Err(AppError::db_error(err));
    };
    tx.commit().await.map_err(AppError::from)?;
    Ok(true)
}
