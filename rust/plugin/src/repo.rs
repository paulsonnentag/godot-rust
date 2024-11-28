use std::{str::FromStr, thread::panicking, time::Duration};

use automerge::{transaction::Transactable, ReadDoc, ScalarValue, Value};
use godot::classes::INode;
use godot::global::godot_print;
use godot::prelude::*;

use automerge_repo::{tokio::FsStorage, ConnDirection, DocumentId, Repo, RepoHandle};
use tokio::{net::TcpStream, runtime::Runtime};

#[derive(GodotClass)]
#[class(no_init, base=Node)]
pub struct AutomergeRepo {
    repo_handle: RepoHandle,
    runtime: Runtime,
}

#[godot_api]
impl AutomergeRepo {
    #[func]
    fn create() -> Gd<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let _guard = runtime.enter();

        println!("outside the spawned thing!");

        godot_print!("init Automerge Repo");

        let _ = tracing_subscriber::fmt::try_init();

        let storage = FsStorage::open("/tmp/automerge-godot-data").unwrap();
        let repo = Repo::new(None, Box::new(storage));
        let repo_handle = repo.run();
        let repo_handle_cloned = repo_handle.clone();

        runtime.spawn(async move {
            println!("inside the spawned thing");

            let document_handle = repo_handle_cloned.new_document();

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

            repo_handle_cloned
                .connect_tokio_io("127.0.0.1:8080", stream, ConnDirection::Outgoing)
                .await
                .unwrap();

            println!("create doc");

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
            repo_handle_cloned.stop().unwrap();
        });

        //godot_print!("done");

        std::thread::sleep(Duration::from_secs(1));

        return Gd::from_init_fn(|base| Self {
            repo_handle,
            runtime,
        });
    }

    #[func]
    fn get_value(&self, doc_id_string: String) {
        let repo_handle = self.repo_handle.clone();
        self.runtime.spawn(async move {
            let doc_id = DocumentId::from_str(&doc_id_string).unwrap();
            let doc_handle = repo_handle.request_document(doc_id);

            let result = doc_handle.await.unwrap();

            result.with_doc(|d| {
                let value: i64 = d
                    .get(automerge::ROOT, "counter")
                    .unwrap()
                    .map(|val| match val {
                        (Value::Scalar(val), automerge::ObjId::Id(_, actor_id, _)) => {
                            match val.as_ref() {
                                ScalarValue::Int(num) => *num,
                                _ => panic!("not a number"),
                            }
                        }
                        _ => panic!("not a number"),
                    })
                    .unwrap_or_default();

                println!("value {:?}", value);

                return;
            });
        });

        std::thread::sleep(Duration::from_secs(1));
    }

    #[func]
    fn inc_value(&self, doc_id_string: String) {
        let repo_handle = self.repo_handle.clone();
        self.runtime.spawn(async move {
            let doc_id = DocumentId::from_str(&doc_id_string).unwrap();
            let doc_handle = repo_handle.request_document(doc_id);

            let result = doc_handle.await.unwrap();

            result.with_doc_mut(|d| {
                let value: i64 = d
                    .get(automerge::ROOT, "counter")
                    .unwrap()
                    .map(|val| match val {
                        (Value::Scalar(val), automerge::ObjId::Id(_, actor_id, _)) => {
                            match val.as_ref() {
                                ScalarValue::Int(num) => *num,
                                _ => panic!("not a number"),
                            }
                        }
                        _ => panic!("not a number"),
                    })
                    .unwrap_or_default();

                let mut tx = d.transaction();
                tx.put(automerge::ROOT, "counter", value + 1)
                    .expect("Failed to change the document.");
                tx.commit();

                return;
            });
        });

        std::thread::sleep(Duration::from_secs(1));
    }
}
