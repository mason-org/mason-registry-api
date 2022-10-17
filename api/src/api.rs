use serde::Serialize;

use crate::github::client::{
    graphql::{tags::TagNode, Edge},
    spec::GitHubRef,
};

#[derive(Serialize)]
pub struct TagResponse {
    pub tag: String,
}

impl From<Edge<TagNode>> for TagResponse {
    fn from(edge: Edge<TagNode>) -> Self {
        Self {
            tag: edge.node.name,
        }
    }
}

impl From<GitHubRef> for TagResponse {
    fn from(github_ref: GitHubRef) -> Self {
        // uuuuhh..
        Self {
            tag: github_ref
                .r#ref
                .strip_prefix("refs/tags/")
                .map(ToOwned::to_owned)
                .unwrap_or(github_ref.r#ref),
        }
    }
}
