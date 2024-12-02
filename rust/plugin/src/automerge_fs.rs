use std::{
    str::FromStr,
    sync::mpsc::{channel, Receiver, Sender},
};

use automerge::{ChangeHash, Patch, ScalarValue};
use autosurgeon::reconcile;
use godot::{obj::WithBaseField, prelude::*};

use automerge::patches::TextRepresentation;
use automerge_repo::{tokio::FsStorage, ConnDirection, DocumentId, Repo, RepoHandle};
use tokio::{net::TcpStream, runtime::Runtime};

use crate::godot_scene;

#[derive(GodotClass)]
#[class(no_init, base=Node)]
pub struct AutomergeFS {
    repo_handle: RepoHandle,
    runtime: Runtime,
    fs_doc_id: DocumentId,
    base: Base<Node>,
    sender: Sender<FileUpdate>,
    receiver: Receiver<FileUpdate>,
}

struct FileUpdate {
    path: String,
    content: String,
}

#[godot_api]
impl AutomergeFS {
    #[signal]
    fn file_changed(path: String, content: String);

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
        let repo_handle_clone = repo_handle.clone();
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

            repo_handle_clone
                .connect_tokio_io("127.0.0.1:8080", stream, ConnDirection::Outgoing)
                .await
                .unwrap();
        });

        let (sender, receiver) = channel::<FileUpdate>();

        return Gd::from_init_fn(|base| Self {
            repo_handle,
            fs_doc_id: doc_id,
            runtime,
            base,
            sender,
            receiver,
        });
    }

    #[func]
    fn stop(&self) {
        self.repo_handle.clone().stop().unwrap();

        // todo: shut down runtime
        //self.runtime.shutdown_background();
    }

    // needs to be called in godot on each frame
    #[func]
    fn refresh(&mut self) {
        let update = self.receiver.try_recv();

        match update {
            Ok(update) => {
                self.base_mut().emit_signal(
                    "file_changed",
                    &[update.path.to_variant(), update.content.to_variant()],
                );
            }
            Err(_) => (),
        }
    }

    #[func]
    fn start(&self) {
        // listen for changes to fs doc
        let repo_handle_change_listener = self.repo_handle.clone();
        let fs_doc_id = self.fs_doc_id.clone();
        let sender = self.sender.clone();

        self.runtime.spawn(async move {
            let doc_handle = repo_handle_change_listener
                .request_document(fs_doc_id)
                .await
                .unwrap();

            let mut heads: Vec<ChangeHash> = vec![];

            loop {
                doc_handle.changed().await.unwrap();

                doc_handle.with_doc(|d| {
                    let new_heads = d.get_heads();
                    let patches = d.diff(&heads, &new_heads, TextRepresentation::String);

                    println!("patches: {:?}", patches);

                    heads = new_heads
                });
            }
        });
    }

    #[func]
    fn save(&self, path: String, content: String) {
        let repo_handle = self.repo_handle.clone();
        let fs_doc_id = self.fs_doc_id.clone();

        println!("save {:?}", path);

        // todo: handle non scene files
        if !path.ends_with(".tscn") {
            return;
        }

        let scene = godot_scene::parse(&content).unwrap();

        self.runtime.spawn(async move {
            let doc_handle = repo_handle.request_document(fs_doc_id);
            let result = doc_handle.await.unwrap();

            result.with_doc_mut(|d| {
                let mut tx = d.transaction();

                let _ = reconcile(&mut tx, scene);
                tx.commit();

                return;
            });
        });
    }
}
