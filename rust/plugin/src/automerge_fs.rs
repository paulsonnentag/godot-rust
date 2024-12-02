use std::{
    str::FromStr,
    sync::mpsc::{channel, Receiver, Sender},
};

use automerge::{transaction::Transactable, ChangeHash, Patch, ScalarValue};
use godot::{obj::WithBaseField, prelude::*};

use automerge::patches::TextRepresentation;
use automerge_repo::{tokio::FsStorage, ConnDirection, DocumentId, Repo, RepoHandle};
use tokio::{net::TcpStream, runtime::Runtime};
use tree_sitter::{Parser, Query, QueryCursor};

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

                    for patch in patches {
                        match patch {
                            Patch {
                                obj: _,
                                path,
                                action,
                            } => {
                                if path.is_empty() {
                                    if let automerge::PatchAction::PutMap {
                                        key,
                                        value: (automerge::Value::Scalar(v), _),
                                        ..
                                    } = action
                                    {
                                        match v.as_ref() {
                                            ScalarValue::Str(smol_str) => {
                                                println!("rust: send {:?}", key);

                                                let _ = sender.send(FileUpdate {
                                                    path: key,
                                                    content: smol_str.to_string(),
                                                });
                                            }
                                            _ => (),
                                        }
                                    }
                                }
                            }
                        }
                    }

                    heads = new_heads
                });
            }
        });
    }

    #[func]
    fn save(&self, path: String, content: String) {
        let repo_handle = self.repo_handle.clone();
        let fs_doc_id = self.fs_doc_id.clone();
        let path_clone = path.clone();
        let content_clone = content.clone();
        let content_clone_2 = content.clone();

        println!("save {:?}", path);

        if path.ends_with(".tscn") {
            let mut parser = Parser::new();
            parser
                .set_language(tree_sitter_godot_resource::language())
                .expect("Error loading godot resource grammar");

            let result = parser.parse(content, None);
            let cloned_result = result.clone();

            /*  let tree = result.unwrap();
            println!("parse tree:");
            println!("{}", tree.root_node().to_sexp()); */

            match cloned_result {
                Some(tree) => {
                    let content_bytes = content_clone_2.as_bytes();

                    // Query for section attributes and paths
                    let query = "(section 
                        (attribute (identifier) @name_id (#eq? @name_id \"name\") (string) @name_value)
                        (attribute (identifier) @parent_id (#eq? @parent_id \"parent\") (string) @parent_value)?
                        (attribute (identifier) @instance_id (#eq? @instance_id \"instance\") (constructor) @instance_value)?
                        (property (path) @path_key (_) @path_value)*
                    )";
                    let query = Query::new(tree_sitter_godot_resource::language(), query)
                        .expect("Invalid query");
                    let mut query_cursor = QueryCursor::new();
                    let matches = query_cursor.matches(&query, tree.root_node(), content_bytes);

                    for m in matches {
                        let mut name = "";
                        let mut parent = "";
                        let mut instance = "";
                        let mut properties = Vec::new();

                        for capture in m.captures {
                            match capture.index {
                                1 => {
                                    // @name_value
                                    if let Ok(val) = capture.node.utf8_text(content_bytes) {
                                        name = val;
                                    }
                                }
                                3 => {
                                    // @parent_value
                                    if let Ok(val) = capture.node.utf8_text(content_bytes) {
                                        parent = val;
                                    }
                                }
                                5 => {
                                    // @instance_value
                                    if let Ok(val) = capture.node.utf8_text(content_bytes) {
                                        instance = val;
                                    }
                                }
                                6 => {
                                    // @path_key
                                    if let Ok(path) = capture.node.utf8_text(content_bytes) {
                                        if let Some(next_capture) =
                                            m.captures.get(capture.index as usize + 1)
                                        {
                                            if let Ok(value) =
                                                next_capture.node.utf8_text(content_bytes)
                                            {
                                                properties
                                                    .push((path.to_string(), value.to_string()));
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }

                        println!(
                            "Section: name={}, parent={}, instance={}, properties={:?}",
                            name, parent, instance, properties
                        );
                    }
                }
                None => println!("invalid"),
            }
        }

        self.runtime.spawn(async move {
            let doc_handle = repo_handle.request_document(fs_doc_id);
            let result = doc_handle.await.unwrap();

            result.with_doc_mut(|d| {
                let mut tx = d.transaction();
                tx.put(automerge::ROOT, path, content_clone)
                    .expect(&format!("Failed to save {:?}", path_clone));
                tx.commit();

                return;
            });
        });
    }
}
