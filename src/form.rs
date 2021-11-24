use serde::Deserialize;

/// 创建待办列表
#[derive(Deserialize)]
pub struct CreateTodoList {
    pub title: String,
}

/// 修改待办列表
#[derive(Deserialize)]
pub struct UpdateTodoList {
    pub id: i32,
    pub title: String,
}

#[derive(Deserialize)]
pub struct CreateTodoItem {
    pub title: String,
    pub list_id: i32,
}
