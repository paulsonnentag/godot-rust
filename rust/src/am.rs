use automerge::{transaction::Transactable, AutoCommit, ReadDoc};
use godot::classes::INode;
use godot::prelude::*;
use std::borrow::Cow;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct AutomergeDoc {
    pub doc: AutoCommit,
}

#[godot_api]
impl INode for AutomergeDoc {
    fn init(base: Base<Node>) -> Self {
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
        match self.doc.get(automerge::ROOT, &key) {
            Ok(Some((val, _))) => val.to_string(),
            _ => String::new(),
        }
    }
}
