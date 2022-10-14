use std::collections::VecDeque;

use vercel_lambda::error::VercelError;

use super::{
    client::{
        graphql::{tags::TagNode, Edge},
        spec::GitHubReleaseDto,
        GitHubClient, GitHubPagination,
    },
    GitHubRepo,
};

pub struct GitHubManager {
    client: GitHubClient,
}

impl GitHubManager {
    pub fn new(client: GitHubClient) -> Self {
        Self { client }
    }

    pub fn get_all_tags(&self, repo: &GitHubRepo) -> Result<Vec<Edge<TagNode>>, VercelError> {
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

    pub fn get_latest_tag(&self, repo: &GitHubRepo) -> Result<Edge<TagNode>, VercelError> {
        let response = self.client.fetch_tags(&repo, Some(1), None)?;
        let mut tags: VecDeque<Edge<TagNode>> = response.data.tags.into();
        let latest_tag = tags
            .pop_front()
            .ok_or_else(|| VercelError::new("Failed to find tag."))?;
        Ok(latest_tag)
    }

    pub fn get_all_releases(
        &self,
        repo: &GitHubRepo,
    ) -> Result<Vec<GitHubReleaseDto>, VercelError> {
        self.client.paginate(
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
        )
    }

    pub fn get_latest_release(
        &self,
        repo: &GitHubRepo,
        include_prerelease: bool,
    ) -> Result<GitHubReleaseDto, VercelError> {
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
        releases.into_iter().find(is_latest_release).ok_or_else(|| {
            VercelError::new(&format!("Unable to find latest release for repo {}.", repo))
        })
    }
}
