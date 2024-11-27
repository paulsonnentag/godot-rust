use automerge_repo::tokio::FsStorage;
use automerge_repo::{ConnDirection, Repo};
use tokio::net::TcpListener;
use tokio::runtime::Handle;

#[tokio::main]
async fn main() {
    let storage = FsStorage::open("/tmp/bla").unwrap();
    let repo = Repo::new(None, Box::new(storage));
    let repo_handle = repo.run();

    let handle = Handle::current();
    let repo_clone = repo_handle.clone();

    handle.spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    repo_clone
                        .connect_tokio_io(addr, socket, ConnDirection::Incoming)
                        .await
                        .unwrap();
                }
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    });

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");
}
