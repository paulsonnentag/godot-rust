use std::process::exit;
use std::time::Duration;

use automerge::{transaction::Transactable, ReadDoc};
use automerge_repo::tokio::FsStorage;
use automerge_repo::{ConnDirection, Repo};
use tokio::net::TcpStream;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tracing_subscriber::fmt::init();
    let storage = FsStorage::open("/tmp/automerge-client-data").unwrap();
    let repo = Repo::new(None, Box::new(storage));
    let repo_handle = repo.run();

    let document_handle = repo_handle.new_document();

    // Start a client.
    let stream = loop {
        // Try to connect to a peer
        let res = TcpStream::connect("127.0.0.1:8080").await;
        if res.is_err() {
            continue;
        }
        break res.unwrap();
    };

    repo_handle
        .connect_tokio_io("127.0.0.1:8080", stream, ConnDirection::Outgoing)
        .await
        .unwrap();

    document_handle.with_doc_mut(|doc| {
        let mut tx = doc.transaction();
        tx.put(automerge::ROOT, "counter", 0)
            .expect("Failed to change the document.");
        tx.commit();
    });

    println!("do");

    document_handle.changed().await.unwrap();

    println!("done");

    tokio::signal::ctrl_c().await.unwrap();
    repo_handle.stop().unwrap();

}
