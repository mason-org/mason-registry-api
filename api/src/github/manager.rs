use std::collections::VecDeque;

use super::{
    client::{
        graphql::{tags::TagNode, Edge},
        spec::{GitHubRef, GitHubReleaseDto},
        GitHubClient, GitHubPagination,
    },
    errors::GitHubError,
    GitHubRepo, GitHubTag,
};

pub struct GitHubManager {
    client: GitHubClient,
}

impl GitHubManager {
    pub fn new(client: GitHubClient) -> Self {
        Self { client }
    }

    /// Returns all tags in DESCENDING order.
    pub fn get_all_tags(&self, repo: &GitHubRepo) -> Result<Vec<Edge<TagNode>>, GitHubError> {
        let mut all_tags: Vec<Edge<TagNode>> = vec![];
        let mut cursor = None;

        loop {
            let response = self.client.fetch_tags(
                &repo,
                Some(GitHubPagination::MAX_PAGE_LIMIT.into()),
                cursor,
            )?;
            cursor = response.data.tags.last().map(|t| t.cursor.to_owned());
            let mut tags = response.data.tags;
            let tags_size = tags.len();
            all_tags.append(&mut tags);

            if tags_size < GitHubPagination::MAX_PAGE_LIMIT.into() {
                return Ok(all_tags);
            }
        }
    }

    pub fn get_latest_tag(&self, repo: &GitHubRepo) -> Result<Edge<TagNode>, GitHubError> {
        let response = self.client.fetch_tags(&repo, Some(1), None)?;
        let mut tags: VecDeque<Edge<TagNode>> = response.data.tags.into();
        let latest_tag = tags
            .pop_front()
            .ok_or_else(|| GitHubError::ResourceNotFound { source: None })?;
        Ok(latest_tag)
    }

    pub fn get_ref(
        &self,
        repo: &GitHubRepo,
        tag: &GitHubTag,
    ) -> Result<GitHubRef, GitHubError> {
        let tag = self.client.fetch_ref(repo, tag)?;
        Ok(tag.data)
    }

    /// Returns all releases in DESCENDING order.
    pub fn get_all_releases(
        &self,
        repo: &GitHubRepo,
    ) -> Result<Vec<GitHubReleaseDto>, GitHubError> {
        Ok(self.client.paginate(
            || {
                self.client.fetch_releases(
                    &repo,
                    Some(GitHubPagination {
                        page: 1,
                        per_page: GitHubPagination::MAX_PAGE_LIMIT,
                    }),
                )
            },
            |_| true,
        )?)
    }

    pub fn get_latest_release(
        &self,
        repo: &GitHubRepo,
        include_prerelease: bool,
    ) -> Result<GitHubReleaseDto, GitHubError> {
        let is_latest_release = |release: &GitHubReleaseDto| {
            if include_prerelease {
                !release.draft
            } else {
                !release.draft && !release.prerelease
            }
        };
        let releases = self.client.paginate(
            || {
                self.client.fetch_releases(
                    &repo,
                    Some(GitHubPagination {
                        page: 1,
                        per_page: GitHubPagination::MAX_PAGE_LIMIT,
                    }),
                )
            },
            |response| {
                (&response.data)
                    .into_iter()
                    .find(|r| is_latest_release(*r))
                    .is_none()
            },
        )?;
        releases
            .into_iter()
            .find(is_latest_release)
            .ok_or_else(|| GitHubError::ResourceNotFound { source: None })
    }

    pub fn get_release_by_tag(
        &self,
        repo: &GitHubRepo,
        release: &GitHubTag,
    ) -> Result<GitHubReleaseDto, GitHubError> {
        Ok(self.client.fetch_release_by_tag(&repo, &release)?.data)
    }
}
