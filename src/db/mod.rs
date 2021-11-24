use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{types::ToSql, GenericClient, Statement};

use crate::{error::AppError, Result};

pub mod todo_item;
pub mod todo_list;

async fn get_stmt<C: GenericClient>(client: &C, sql: &str) -> Result<Statement> {
    client.prepare(sql).await.map_err(AppError::from)
}

async fn query<T, C>(client: &C, sql: &str, args: &[&(dyn ToSql + Sync)]) -> Result<Vec<T>>
where
    C: GenericClient,
    T: FromTokioPostgresRow,
{
    let stmt = get_stmt(client, sql).await?;
    let result = client
        .query(&stmt, args)
        .await
        .map_err(AppError::from)?
        .iter()
        .map(|row| <T>::from_row_ref(row).unwrap())
        .collect::<Vec<T>>();
    Ok(result)
}

async fn query_one<T, C>(client: &C, sql: &str, args: &[&(dyn ToSql + Sync)]) -> Result<T>
where
    C: GenericClient,
    T: FromTokioPostgresRow,
{
    let result: T = query(client, sql, args)
        .await?
        .pop()
        .ok_or(AppError::not_found())?;
    Ok(result)
}

async fn execute<C: GenericClient>(
    client: &C,
    sql: &str,
    args: &[&(dyn ToSql + Sync)],
) -> Result<u64> {
    let stmt = get_stmt(client, sql).await?;
    let rows = client.execute(&stmt, args).await.map_err(AppError::from)?;
    Ok(rows)
}
