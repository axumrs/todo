use axum::Json;

use crate::{Response, Result};

pub async fn usage<'a>() -> Result<Json<Response<Vec<&'a str>>>> {
    let data = r#"
        GET /todo -- 获取所有待办列表
        POST /todo -- 添加待办列表
        GET /todo/:list_id -- 获取待办列表详情
        DELETE /todo/:list_id -- 删除指定的待办列表，包括其所有待办事项
        PUT /todo/:list_id -- 修改待办列表
        GET /todo/:list_id/items -- 获取待办列表的所有待办事项
        GET /todo/:list_id/items/:item_id -- 获取待办事项的详情
        PUT /todo/:list_id/items/:item_id -- 修改待办事项（将其的状态修改为“已完成”）
        DELETE /todo/:list_id/items/:item_id -- 删除待办事项
    "#;
    let data: Vec<&str> = data
        .split('\n')
        .into_iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    let data = Response::ok(data);
    Ok(Json(data))
}
