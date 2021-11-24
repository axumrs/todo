//! 自定义错误

use std::{convert::Infallible, fmt::Display};

use axum::{
    body::{Bytes, Full},
    response::IntoResponse,
    Json,
};

use crate::Response;

/// 错误的类型
#[derive(Debug)]
pub enum AppErrorType {
    /// 数据库错误
    DbError,
    /// 未找到
    NotFound,
}

/// 应用错误
#[derive(Debug)]
pub struct AppError {
    /// 错误信息
    pub message: Option<String>,
    /// 错误原因（上一级的错误）
    pub cause: Option<String>,
    /// 错误类型
    pub error_type: AppErrorType,
}

impl AppError {
    /// 错误代码
    fn code(&self) -> i32 {
        match self.error_type {
            AppErrorType::DbError => 1,
            AppErrorType::NotFound => 2,
        }
    }
    /// 从上级错误中创建应用错误
    fn from_err(err: impl ToString, error_type: AppErrorType) -> Self {
        Self {
            message: None,
            cause: Some(err.to_string()),
            error_type,
        }
    }
    /// 从字符串创建应用错误
    fn from_str(msg: &str, error_type: AppErrorType) -> Self {
        Self {
            message: Some(msg.to_string()),
            cause: None,
            error_type,
        }
    }
    /// 数据库错误
    pub fn db_error(err: impl ToString) -> Self {
        Self::from_err(err, AppErrorType::DbError)
    }
    /// 未找到
    pub fn not_found() -> Self {
        Self::from_str("不存在的记录", AppErrorType::NotFound)
    }
}
impl std::error::Error for AppError {}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::db_error(err)
    }
}
impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::db_error(err)
    }
}

/// 实现 IntoResponse
impl IntoResponse for AppError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        let code = (&self).code();
        let msg = match self.message {
            Some(msg) => msg,
            None => "有错误发生".to_string(),
        };
        let res: Response<()> = Response::err(code, msg);
        Json(res).into_response()
    }
}
