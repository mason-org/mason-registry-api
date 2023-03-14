use serde::Deserialize;

pub mod tags;
pub mod sponsors;

#[derive(Debug, Deserialize)]
pub struct Edge<Node> {
    pub cursor: String,
    pub node: Node,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}
