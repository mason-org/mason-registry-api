use std::collections::VecDeque;

use super::{
    client::{
        graphql::{sponsors::Sponsor, tags::Tag},
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
    pub fn get_all_tags(&self, repo: &GitHubRepo) -> Result<Vec<Tag>, GitHubError> {
        let mut all_tags: Vec<Tag> = vec![];
        let mut cursor = None;

        loop {
            let response =
                self.client
                    .fetch_tags(&repo, GitHubPagination::MAX_PAGE_LIMIT.into(), cursor)?;
            let mut tags = response.data.tags;
            all_tags.append(&mut tags);

            // cursor = response.data.page_info.end_cursor;
            // if !response.data.page_info.has_next_page {
            cursor = None;
            if true {
                return Ok(all_tags);
            }
        }
    }

    pub fn get_all_sponsors(&self, login: String) -> Result<Vec<Sponsor>, GitHubError> {
        let mut all_sponsors: Vec<Sponsor> = vec![];
        let mut cursor = None;

        loop {
            let response = self.client.fetch_sponsors(
                login.clone(),
                GitHubPagination::MAX_PAGE_LIMIT.into(),
                cursor,
            )?;
            let mut sponsors = response.data.sponsors;
            all_sponsors.append(&mut sponsors);

            // cursor = response.data.page_info.end_cursor;
            // if !response.data.page_info.has_next_page {
            cursor = None;
            if true {
                return Ok(all_sponsors);
            }
        }
    }

    pub fn get_latest_tag(&self, repo: &GitHubRepo) -> Result<Tag, GitHubError> {
        let response = self.client.fetch_tags(&repo, 1, None)?;
        let mut tags: VecDeque<Tag> = response.data.tags.into();
        let latest_tag = tags
            .pop_front()
            .ok_or_else(|| GitHubError::ResourceNotFound { source: None })?;
        Ok(latest_tag)
    }

    pub fn get_ref(&self, repo: &GitHubRepo, tag: &GitHubTag) -> Result<GitHubRef, GitHubError> {
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
