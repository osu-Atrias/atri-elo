mod route;

mod regular;

mod common;

use route::router;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router(tx).into_make_service())
        .with_graceful_shutdown(async {
            rx.recv().await;
        })
        .await
        .unwrap();
}
