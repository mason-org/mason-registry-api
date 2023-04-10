use chrono::{DateTime, Utc};

use crate::{
    badges::{Badge, BadgeColor},
    github::GitHubRepo,
};

use super::{
    client::{spec::JobResult, RenovateClient},
    errors::RenovateError,
};

pub struct RenovateManager {
    client: RenovateClient,
}

impl RenovateManager {
    pub fn new(client: RenovateClient) -> Self {
        Self { client }
    }

    pub fn get_badge(&self, repo: &GitHubRepo, style: BadgeColor) -> Result<Badge, RenovateError> {
        let jobs = self.client.fetch_github_jobs(repo)?.jobs;
        if let Some(job) = jobs.iter().rev().find(|job| job.result == JobResult::Done) {
            let date_time = DateTime::parse_from_rfc3339(&job.ended).map_err(|err| {
                tracing::error!("Failed to parse job ended timestamp {}: {}", job.ended, err);
                RenovateError::InternalError
            })?;
            let now = Utc::now();
            let duration = now.signed_duration_since(date_time);

            Ok(Badge::new(
                "renovate".to_owned(),
                format!(
                    "{} minutes ago",
                    duration.num_minutes()
                ),
                style,
            ))
        } else {
            tracing::debug!("Unable to find any done jobs {:?}", jobs);
            Err(RenovateError::ResourceNotFound { source: None })
        }
    }
}
