use shared::config::DatabaseConfig;
use sqlx::{postgres::PgConnectOptions, PgPool};

// 1) `DatabaseConfig`から`PgConnectOptions`に変換する関数
fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

// 2) `sqlx::PgPool`をラップする
#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    // 3) `sqlx::PgPool`への参照を取得する
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

// 4) 返り値を`ConnectionPool`に変更し、内部実装もそれに合わせて修正した
pub fn connect_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(cfg)))
}
