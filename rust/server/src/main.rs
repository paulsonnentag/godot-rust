use automerge_repo::tokio::FsStorage;
use automerge_repo::{ConnDirection, Repo};
use tokio::net::TcpListener;
use tokio::runtime::Handle;
use tracing_subscriber;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tracing_subscriber::fmt::init();

    let storage = FsStorage::open("/tmp/automerge-server-data").unwrap();
    let repo = Repo::new(None, Box::new(storage));
    let repo_handle = repo.run();

    let handle = Handle::current();

    let repo_clone = repo_handle.clone();
    handle.spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    println!("client connected");
                    repo_clone
                        .connect_tokio_io(addr, socket, ConnDirection::Incoming)
                        .await
                        .unwrap();
                }
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    });

    tokio::signal::ctrl_c().await.unwrap();

    repo_handle.stop().unwrap();
}
