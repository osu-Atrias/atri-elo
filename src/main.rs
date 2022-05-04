use std::{path::Path, borrow::Cow};

use chrono::Utc;
use libmdbx::{Environment, WriteMap, WriteFlags, NoWriteMap};
use model::Player;
use route::router;
use speedy::{Writable, Readable};

mod route;

mod model;

mod util;

#[tokio::main]
async fn main() {
    // let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    //     .serve(router(tx).into_make_service())
    //     .with_graceful_shutdown(async {
    //         rx.recv().await;
    //     })
    //     .await
    //     .unwrap();

    // let data = Player::new(1, "arc".to_string(), 15.0, 2.0);
    // let bytes = data.write_to_vec().unwrap();
    // let env = Environment::<WriteMap>::new()
    //     .open(Path::new("test.db"))
    //     .unwrap();
    // let txn = env.begin_rw_txn().unwrap();
    // let db = txn.open_db(None).unwrap();
    // txn.put(&db, [1], bytes.as_slice(), WriteFlags::default()).unwrap();
    // txn.commit().unwrap();

    let start = Utc::now();
    let env = Environment::<WriteMap>::new()
        .open(Path::new("test.db"))
        .unwrap();
    println!("open time: {} μs", (Utc::now() - start).num_nanoseconds().unwrap() as f64 / 1000.0);
    let start = Utc::now();
    let txn = env.begin_rw_txn().unwrap();
    let db = txn.open_db(None).unwrap();
    let data: Cow<[u8]> = txn.get(&db, &[1]).unwrap().unwrap();
    println!("read time: {} μs", (Utc::now() - start).num_nanoseconds().unwrap() as f64 / 1000.0);
    let start = Utc::now();
    let archived = Player::read_from_buffer(&data).unwrap();
    println!("des time: {} μs", (Utc::now() - start).num_nanoseconds().unwrap() as f64 / 1000.0);
    let stats = txn.db_stat(&db).unwrap();
    println!("page_size: {}", stats.page_size());
    println!("depth: {}", stats.depth());
    println!("branch_pages: {}", stats.branch_pages());
    println!("leaf_pages: {}", stats.leaf_pages());
    println!("overflow_pages: {}", stats.overflow_pages());
    println!("entries: {}", stats.entries());
}
