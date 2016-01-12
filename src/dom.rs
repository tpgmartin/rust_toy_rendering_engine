//! Basic DOM data structure

use std::collections::HashMap;

struct Node {
    // data common to all nodes:
    children: Vec<node>,
    
    // data specific to each node type:
    node_type: NodeType,
}

// define NodeType
enum NodeType {
    Text(String),
    Element(ElementData),
}

type AttrMap = HashMap<String, String>;

// constructor functions
fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

fn elem(name: String, attrs: AttrMap, children: Vec<node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs
        })
    }
}
