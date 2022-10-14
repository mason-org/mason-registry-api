use serde::Deserialize;

pub mod tags;

#[derive(Debug, Deserialize)]
pub struct Edge<Node> {
    pub cursor: String,
    pub node: Node,
}
