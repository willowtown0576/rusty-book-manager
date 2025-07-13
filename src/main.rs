use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

use sqlx::{postgres::PgConnectOptions, PgPool};

// データベースの接続設定を表す構造体を定義する
struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

// アプリケーション用のデータベース設定構造体から、Postgres接続用の構造体へ変換する
impl From<DatabaseConfig> for PgConnectOptions {
    fn from(cfg: DatabaseConfig) -> Self {
        Self::new()
            .host(&cfg.host)
            .port(cfg.port)
            .username(&cfg.username)
            .password(&cfg.password)
            .database(&cfg.database)
    }
}

// Postgres専用のコネクションプールを作成する
fn connect_database_with(cfg: DatabaseConfig) -> PgPool {
    PgPool::connect_lazy_with(cfg.into())
}

// ヘルスチェック用のハンドラ
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

// 非同期処理のテスト実行には、tokio::testマクロを使用する
#[tokio::test]
async fn health_check_works() {
    assert_eq!(health_check().await, StatusCode::OK);
}

// データベースヘルスチェック用のハンドラ
async fn health_check_db(State(db): State<PgPool>) -> StatusCode {
    let connection_result = sqlx::query("SELECT 1")
        .fetch_one(&db)
        .await;
    match connection_result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[sqlx::test]
async fn health_check_db_works(pool: PgPool) {
    let status_code = health_check_db(State(pool)).await;
    assert_eq!(status_code, StatusCode::OK);
}

#[tokio::main]
async fn main() -> Result<()> {
    // データベース接続設定を定義する
    let database_cfg = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        username: "app".to_string(),
        password: "passwd".to_string(),
        database: "app".to_string(),
    };

    // コネクションプールを作る
    let conn_pool = connect_database_with(database_cfg);

    let app = Router::new()
        .route("/health", get(health_check))
        // ルーターにデータベースチェック用のハンドラを登録する
        .route("/health/db", get(health_check_db))
        // ルーターの`State`にプールを登録しておき、各ハンドラで使えるようにする
        .with_state(conn_pool);
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    Ok(axum::serve(listener, app).await?)
}
