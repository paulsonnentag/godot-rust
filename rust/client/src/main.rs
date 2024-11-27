use std::process::exit;

use automerge::{transaction::Transactable, ReadDoc};
use automerge_repo::tokio::FsStorage;
use automerge_repo::{ConnDirection, Repo};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let storage = FsStorage::open("/tmp/automerge-client-data").unwrap();
    let repo = Repo::new(None, Box::new(storage));
    let repo_handle = repo.run();

    // Start a client.
    // Spawn a task connecting to the other peer.
    let stream = loop {
        // Try to connect to a peer
        let res = TcpStream::connect("127.0.0.1:8080").await;
        if res.is_err() {
            continue;
        }
        break res.unwrap();
    };

    let task = tokio::spawn({
        let repo_handle = repo_handle.clone();
        async move {
            repo_handle
                .connect_tokio_io("127.0.0.1:8080", stream, ConnDirection::Outgoing)
                .await
                .unwrap()
        }
    });

    let document_handle = repo_handle.new_document();

    // Spawn a task that makes a change the document change.
    tokio::spawn({
        let document_handle_clone = document_handle.clone();
        async move {
            // Edit the document.
            document_handle_clone.with_doc_mut(|doc| {
                let mut tx = doc.transaction();
                tx.put(automerge::ROOT, "counter", 0)
                    .expect("Failed to change the document.");
                tx.commit();
            });
        }
    });

    println!("do");

    document_handle.changed().await.unwrap();

    println!("done");

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("ctrl-c received");
            repo_handle.stop().unwrap()
        },
        listen_res = task => {
            println!("listen task finished unexpectedly: {:?}", listen_res);
            repo_handle.stop().unwrap();
            exit(0)
        }
    };

}
