use axum::Json;
use deadpool_postgres::Client;

use crate::{error::AppError, model::AppState, Response, Result};

pub mod todo_item;
pub mod todo_list;
pub mod usage;

type HandlerResult<T> = crate::Result<Json<Response<T>>>;

/// 获取数据库连接，如果发生错误，则记录错误到日志中
async fn get_client(state: &AppState, handler_name: &str) -> Result<Client> {
    state.pool.get().await.map_err(|err| {
        tracing::error!("{}: {:?}", handler_name, err);
        AppError::db_error(err)
    })
}

/// 记录日志
fn log_error(handler_name: String) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        tracing::debug!("{}: {:?}", handler_name, err);
        err
    })
}
