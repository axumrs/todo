use tokio_postgres::Client;

use crate::{
    error::AppError,
    form,
    model::{TodoList, TodoListID},
    Result,
};

/// 创建待办列表
pub async fn create(client: &Client, frm: form::CreateTodoList) -> Result<TodoListID> {
    let result: TodoListID = super::query_one(
        client,
        "INSERT INTO todo_list (title) VALUES ($1) RETURNING id",
        &[&frm.title],
    )
    .await?;
    Ok(result)
}

/// 所有待办列表
pub async fn all(client: &Client) -> Result<Vec<TodoList>> {
    let result: Vec<TodoList> = super::query(
        client,
        "SELECT id,title FROM todo_list ORDER BY id DESC",
        &[],
    )
    .await?;
    Ok(result)
}

/// 根据ID查找待办列表
pub async fn find(client: &Client, list_id: i32) -> Result<TodoList> {
    let result: TodoList = super::query_one(
        client,
        "SELECT id,title FROM todo_list WHERE id=$1 LIMIT 1",
        &[&list_id],
    )
    .await?;
    Ok(result)
}

/// 更新待办列表
pub async fn update(client: &Client, frm: form::UpdateTodoList) -> Result<bool> {
    let result = super::execute(
        client,
        "UPDATE todo_list SET title=$1 WHERE id=$2",
        &[&frm.title, &frm.id],
    )
    .await?;
    Ok(result > 0)
}

/// 删除待办列表
pub async fn delete(client: &mut Client, id: i32) -> Result<bool> {
    let tx = client.transaction().await.map_err(AppError::from)?;
    let result = super::execute(&tx, "DELETE FROM todo_list  WHERE id=$1", &[&id]).await;
    if let Err(err) = result {
        tx.rollback().await.map_err(AppError::from)?;
        return Err(AppError::db_error(err));
    };
    let result = super::execute(&tx, "DELETE FROM todo_item WHERE list_id=$1", &[&id]).await;
    if let Err(err) = result {
        tx.rollback().await.map_err(AppError::from)?;
        return Err(AppError::db_error(err));
    };
    tx.commit().await.map_err(AppError::from)?;
    Ok(true)
}
