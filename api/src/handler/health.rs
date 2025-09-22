use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

// ヘルスチェック用のハンドラ
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

// 1) `State`におつろくされている`AppRegistry`を取り出す
pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    // 2) health_check_repositoryメソッドを経由してリポジトリの処理を呼び出せる
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
