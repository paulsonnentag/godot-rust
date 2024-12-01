use autosurgeon::{Hydrate, Reconcile};
use std::collections::HashMap;
use tree_sitter::{Parser, Query, QueryCursor};

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq)]
pub struct PackedGodotScene {
    // todo: parse  resources and connections
    nodes: std::collections::HashMap<String, GodotSceneNode>,
}

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq)]
pub struct GodotSceneNode {
    name: String,
    parent: String,
    instance: String,
    props: HashMap<String, String>,
}

pub fn parse(source: &String) -> Result<PackedGodotScene, String> {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_godot_resource::language())
        .expect("Error loading godot resource grammar");

    let result = parser.parse(source, None);

    return match result {
        Some(tree) => {
            let content_bytes = source.as_bytes();

            // Query for section attributes and paths
            let query = "(section 
              (attribute (identifier) @name_id (#eq? @name_id \"name\") (string) @name_value)
              (attribute (identifier) @parent_id (#eq? @parent_id \"parent\") (string) @parent_value)?
              (attribute (identifier) @instance_id (#eq? @instance_id \"instance\") (constructor) @instance_value)?
              (property (path) @path_key (_) @path_value)*
          )";
            let query =
                Query::new(tree_sitter_godot_resource::language(), query).expect("Invalid query");
            let mut query_cursor = QueryCursor::new();
            let matches = query_cursor.matches(&query, tree.root_node(), content_bytes);
            let mut scene = PackedGodotScene {
                nodes: std::collections::HashMap::new(),
            };

            for m in matches {
                let mut name = "";
                let mut parent = "";
                let mut instance = "";
                let mut props = HashMap::new();

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
                                    if let Ok(value) = next_capture.node.utf8_text(content_bytes) {
                                        props.insert(path.to_string(), value.to_string());
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                let node = GodotSceneNode {
                    name: name.to_string(),
                    parent: parent.to_string(),
                    instance: instance.to_string(),
                    props,
                };

                scene.nodes.insert(name.to_string(), node);
            }

            Ok(scene)
        }
        None => Err("Failed to parse scene file".to_string()),
    };
}
