use std::str::FromStr;

use automerge::{transaction::Transactable, ChangeHash, Patch};
use godot::{global::print, prelude::*};

use automerge::patches::TextRepresentation;
use automerge_repo::{tokio::FsStorage, ConnDirection, DocumentId, Repo, RepoHandle};
use tokio::{net::TcpStream, runtime::Runtime};

#[derive(GodotClass)]
#[class(no_init, base=Node)]
pub struct AutomergeFS {
    repo_handle: RepoHandle,
    runtime: Runtime,
    fs_doc_id: DocumentId,
    base: Base<Node>,
}

#[godot_api]
impl AutomergeFS {
    #[signal]
    fn changed(path: String, content: String);

    #[func]
    fn create(fs_doc_id: String) -> Gd<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let _guard = runtime.enter();

        let _ = tracing_subscriber::fmt::try_init();

        let storage = FsStorage::open("/tmp/automerge-godot-data").unwrap();
        let repo = Repo::new(None, Box::new(storage));
        let repo_handle = repo.run();
        let doc_id = DocumentId::from_str(&fs_doc_id).unwrap();

        // connect repo
        let repo_handle_init = repo_handle.clone();
        runtime.spawn(async move {
            println!("start a client");

            // Start a client.
            let stream = loop {
                // Try to connect to a peer
                let res = TcpStream::connect("127.0.0.1:8080").await;
                if let Err(e) = res {
                    println!("error connecting: {:?}", e);
                    continue;
                }
                break res.unwrap();
            };

            println!("connect repo");

            repo_handle_init
                .connect_tokio_io("127.0.0.1:8080", stream, ConnDirection::Outgoing)
                .await
                .unwrap();
        });

        // listen for changes to fs doc
        let repo_handle_change_listener = repo_handle.clone();
        let doc_id_clone = doc_id.clone();
        runtime.spawn(async move {
            let doc_handle = repo_handle_change_listener
                .request_document(doc_id)
                .await
                .unwrap();

            let mut heads: Vec<ChangeHash> = vec![];

            loop {
                doc_handle.changed().await.unwrap();

                println!("fs changed");

                doc_handle.with_doc(|d| {
                    let new_heads = d.get_heads();
                    let patches = d.diff(&heads, &new_heads, TextRepresentation::String);

                    for patch in patches {
                        match patch {
                            Patch {
                                obj: _,
                                path,
                                action,
                            } => {
                                if path.is_empty() {
                                    match action {
                                        automerge::PatchAction::PutMap {
                                            key,
                                            value: _,
                                            conflict: _,
                                        } => {
                                            println!("remote change {:?}", &key);
                                        }
                                        _ => todo!(),
                                    }
                                }
                            }
                            _ => (),
                        }

                        // println!("patch: {:?}", patch);
                    }

                    heads = new_heads
                });
            }
        });

        return Gd::from_init_fn(|base| Self {
            repo_handle,
            fs_doc_id: doc_id_clone,
            runtime,
            base,
        });
    }

    #[func]
    fn stop(&self) {
        self.repo_handle.clone().stop().unwrap();

        // todo: shut down runtime
        //self.runtime.shutdown_background();
    }

    #[func]
    fn save(&self, path: String, content: String) {
        let repo_handle = self.repo_handle.clone();
        let fs_doc_id = self.fs_doc_id.clone();
        let path_clone = path.clone();

        self.runtime.spawn(async move {
            let doc_handle = repo_handle.request_document(fs_doc_id);
            let result = doc_handle.await.unwrap();

            result.with_doc_mut(|d| {
                let mut tx = d.transaction();
                tx.put(automerge::ROOT, path, content)
                    .expect(&format!("Failed to save {:?}", path_clone));
                tx.commit();

                return;
            });
        });
    }
}
