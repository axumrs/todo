use tokio_postgres::Client;

use crate::{
    form,
    model::{TodoItem, TodoItemID},
    Result,
};

use super::query_one;

pub async fn all(client: &Client, list_id: i32) -> Result<Vec<TodoItem>> {
    let result: Vec<TodoItem> = super::query(
        client,
        "SELECT id,title,checked,list_id FROM todo_item WHERE list_id=$1 ORDER BY id ASC",
        &[&list_id],
    )
    .await?;
    Ok(result)
}

pub async fn find(client: &Client, list_id: i32, item_id: i32) -> Result<TodoItem> {
    let result: TodoItem = super::query_one(
        client,
        "SELECT id,title,checked,list_id FROM todo_item WHERE id=$1 AND list_id=$2",
        &[&item_id, &list_id],
    )
    .await?;
    Ok(result)
}

pub async fn check(client: &Client, list_id: i32, item_id: i32) -> Result<bool> {
    let result = super::execute(
        client,
        "UPDATE todo_item SET checked=true WHERE id=$1 AND list_id=$2 AND checked=false",
        &[&item_id, &list_id],
    )
    .await?;
    Ok(result > 0)
}

pub async fn delete(client: &Client, list_id: i32, item_id: i32) -> Result<bool> {
    let result = super::execute(
        client,
        "DELETE FROM todo_item WHERE id=$1 AND list_id=$2",
        &[&item_id, &list_id],
    )
    .await?;
    Ok(result > 0)
}

pub async fn create(client: &Client, frm: form::CreateTodoItem) -> Result<TodoItemID> {
    let result = query_one(
        client,
        "INSERT INTO todo_item (title, checked, list_id) VALUES ($1,$2,$3) RETURNING id",
        &[&frm.title, &false, &frm.list_id],
    )
    .await?;
    Ok(result)
}
