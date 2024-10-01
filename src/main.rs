mod controller;
use axum::{routing::post, Router};

#[tokio::main]
async fn main() {
    // ルートを定義
    // "/"を踏むと、上で定義したroot_handlerを実行する
    let app = Router::new().route("/", post(controller::git::root_handler));

    // 指定したポートにサーバを開く
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}