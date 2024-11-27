use godot::classes::INode;
use godot::global::godot_print;
use godot::prelude::*;

use automerge_repo::{fs_store, share_policy::ShareDecision, DocumentId, Repo, Storage};
use tokio::runtime;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct AutomergeRepo {
    base: Base<Node>,
    runtime: tokio::runtime::Runtime,
    repo: Repo,
}

#[godot_api]
impl INode for AutomergeRepo {
    fn init(base: Base<Node>) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
        let _guard = runtime.enter();
        let storage = automerge_repo::tokio::FsStorage::open("/tmp/bla").unwrap();
        let repo = Repo::new(None, Box::new(storage));

        runtime.spawn(async move {
            let listener = TcpListener::bind(run_ip).await.unwrap();
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

        return Self {
            repo,
            base,
            runtime,
        };
    }
}

#[godot_api]
impl AutomergeRepo {
    #[func]
    fn destroy(&self) {
        self.runtime.shutdown_background();
    }
}
