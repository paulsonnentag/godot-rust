use automerge::ChangeHash;
use automerge::{transaction::Transactable, AutoCommit, ReadDoc, ScalarValue, Value};
use godot::classes::INode;
use godot::prelude::*;
use std::str::FromStr;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct AutomergeDoc {
    pub doc: AutoCommit,

    base: Base<Node>,
}

#[godot_api]
impl INode for AutomergeDoc {
    fn init(base: Base<Node>) -> Self {
        let doc = AutoCommit::new();
        Self { doc, base }
    }
}

#[godot_api]
impl AutomergeDoc {
    #[signal]
    fn changed();

    #[func]
    fn set(&mut self, key: String, value: String) {
        self.doc.put(automerge::ROOT, key, value);
        self.base_mut().emit_signal("changed", &[]);
    }

    #[func]
    fn get(&self, key: String) -> String {
        self.doc
            .get(automerge::ROOT, &key)
            .unwrap()
            .map(|val| match val {
                (Value::Scalar(val), automerge::ObjId::Id(_, actor_id, _)) => match val.as_ref() {
                    ScalarValue::Str(smol_str) => smol_str.to_string(),
                    _ => panic!("not a string"),
                },
                _ => panic!("not a string"),
            })
            .unwrap_or_default()
    }

    #[func]
    fn get_at(&self, key: String, hash: String) -> String {
        self.doc
            .get_at(
                automerge::ROOT,
                &key,
                &[ChangeHash::from_str(&hash).expect("invalid hash")],
            )
            .unwrap()
            .map(|val| match val {
                (Value::Scalar(val), automerge::ObjId::Id(_, actor_id, _)) => match val.as_ref() {
                    ScalarValue::Str(smol_str) => smol_str.to_string(),
                    _ => panic!("not a string"),
                },
                _ => panic!("not a string"),
            })
            .unwrap_or_default()
    }

    #[func]
    fn history(&mut self) -> Vec<GString> {
        self.doc
            .get_changes(&[])
            .into_iter()
            .map(|c| GString::from(c.hash().to_string()))
            .collect::<Vec<_>>()
    }
}
