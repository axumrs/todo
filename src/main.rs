use axum::{routing::get, AddExtensionLayer, Router};
use dotenv::dotenv;

mod config;
mod db;
mod error;
mod form;
mod handler;
mod model;
mod response;

/// 定义自己的 Result
type Result<T> = std::result::Result<T, error::AppError>;

use model::AppState;
pub use response::Response;

#[tokio::main]
async fn main() {
    // 初始化日志
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "todo=debug");
    }
    tracing_subscriber::fmt::init();

    // 解析 .env 文件
    dotenv().ok();

    let cfg = config::Config::from_env().expect("初始化配置失败");
    let pool = cfg
        .pg
        .create_pool(tokio_postgres::NoTls)
        .expect("初始化数据库连接池失败");

    let app = Router::new()
        .route("/", get(handler::usage::usage))
        .route(
            "/todo",
            get(handler::todo_list::all).post(handler::todo_list::create),
        )
        .route(
            "/todo/:list_id",
            get(handler::todo_list::find)
                .put(handler::todo_list::update)
                .delete(handler::todo_list::delete),
        )
        .route(
            "/todo/:list_id/items",
            get(handler::todo_item::all).post(handler::todo_item::create),
        )
        .route(
            "/todo/:list_id/items/:item_id",
            get(handler::todo_item::find)
                .put(handler::todo_item::check)
                .delete(handler::todo_item::delete),
        )
        .layer(AddExtensionLayer::new(AppState { pool }));

    tracing::info!("服务器监听于：{}", &cfg.web.addr);

    // 绑定到配置文件设置的地址
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
