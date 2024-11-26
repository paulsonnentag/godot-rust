use automerge::{transaction::Transactable, AutoCommit, ReadDoc, ScalarValue, Value};
use godot::classes::INode;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct AutomergeDoc {
    pub doc: AutoCommit,
}

#[godot_api]
impl INode for AutomergeDoc {
    fn init(_base: Base<Node>) -> Self {
        let doc = AutoCommit::new();
        Self { doc }
    }
}

#[godot_api]
impl AutomergeDoc {
    #[func]
    fn set(&mut self, key: String, value: String) {
        self.doc.put(automerge::ROOT, key, value);
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
    fn history(&mut self) -> Vec<GString> {
        self.doc
            .get_changes(&[])
            .into_iter()
            .map(|c| GString::from(c.hash().to_string()))
            .collect::<Vec<_>>()
    }
}
