use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

/// 应用状态共享
#[derive(Clone)]
pub struct AppState {
    /// PostgreSQL 连接池
    pub pool: deadpool_postgres::Pool,
}

/// 待办列表模型
#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "todo_list")]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

/// 待办列表新ID模型
#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "todo_list")]
pub struct TodoListID {
    pub id: i32,
}
