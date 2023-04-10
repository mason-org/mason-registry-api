use std::fmt::Display;

use chrono::{DateTime, Duration, Utc};

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

#[derive(Debug, Eq, PartialEq)]
enum RelativeTimestamp {
    InMinutes(Duration),
    InHours(Duration),
}

impl Display for RelativeTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativeTimestamp::InMinutes(duration) => match duration.num_minutes() {
                0 => f.write_str("just now"),
                1 => f.write_str("1 minute ago"),
                minutes => f.write_fmt(format_args!("{} minutes ago", minutes)),
            },
            RelativeTimestamp::InHours(duration) => match duration.num_hours() {
                1 => f.write_str("1 hour ago"),
                hours => f.write_fmt(format_args!("{} hours ago", hours)),
            },
        }
    }
}

impl RenovateManager {
    pub fn new(client: RenovateClient) -> Self {
        Self { client }
    }

    fn get_duration_since<D1, D2>(dt1: D1, dt2: D2) -> Duration
    where
        D1: Into<DateTime<Utc>>,
        D2: Into<DateTime<Utc>>,
    {
        dt2.into()
            .signed_duration_since(dt1.into())
            .max(Duration::seconds(0))
    }

    fn get_relative_timestamp(delta: Duration) -> RelativeTimestamp {
        match delta {
            delta if delta.num_minutes() < 60 => RelativeTimestamp::InMinutes(delta),
            delta => RelativeTimestamp::InHours(delta),
        }
    }

    fn get_badge_color(duration: Duration) -> BadgeColor {
        match duration.num_hours() {
            0..=7 => BadgeColor::Brightgreen,
            8..=11 => BadgeColor::Yellowgreen,
            12..=16 => BadgeColor::Orange,
            _ => BadgeColor::Red,
        }
    }

    pub fn get_badge(&self, repo: &GitHubRepo) -> Result<Badge, RenovateError> {
        let jobs = self.client.fetch_github_jobs(repo)?.jobs;
        if let Some(job) = jobs.iter().rev().find(|job| job.result == JobResult::Done) {
            let date_time = DateTime::parse_from_rfc3339(&job.ended).map_err(|err| {
                tracing::error!("Failed to parse job ended timestamp {}: {}", job.ended, err);
                RenovateError::InternalError
            })?;
            let delta = Self::get_duration_since(date_time, Utc::now());
            let rel_ts = Self::get_relative_timestamp(delta);

            Ok(Badge::new(
                "renovate".to_owned(),
                rel_ts.to_string(),
                Self::get_badge_color(delta),
            ))
        } else {
            tracing::debug!("Unable to find any done jobs {:?}", jobs);
            Err(RenovateError::ResourceNotFound { source: None })
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Duration, ParseError};

    use super::{RelativeTimestamp, RenovateManager};

    #[test]
    fn should_calculate_duration_since() -> Result<(), ParseError> {
        assert_eq!(
            Duration::minutes(42),
            RenovateManager::get_duration_since(
                DateTime::parse_from_rfc3339("2023-04-10T12:55:00Z")?,
                DateTime::parse_from_rfc3339("2023-04-10T13:37:00Z")?,
            )
        );
        assert_eq!(
            Duration::minutes(59),
            RenovateManager::get_duration_since(
                DateTime::parse_from_rfc3339("2023-04-10T12:38:00Z")?,
                DateTime::parse_from_rfc3339("2023-04-10T13:37:00Z")?,
            )
        );
        assert_eq!(
            Duration::hours(1),
            RenovateManager::get_duration_since(
                DateTime::parse_from_rfc3339("2023-04-10T12:37:00Z")?,
                DateTime::parse_from_rfc3339("2023-04-10T13:37:00Z")?,
            )
        );
        assert_eq!(
            Duration::hours(8760),
            RenovateManager::get_duration_since(
                DateTime::parse_from_rfc3339("2022-04-10T13:37:00Z")?,
                DateTime::parse_from_rfc3339("2023-04-10T13:37:00Z")?,
            )
        );
        Ok(())
    }

    #[test]
    fn should_account_for_clock_skew() -> Result<(), ParseError> {
        assert_eq!(
            Duration::seconds(0),
            RenovateManager::get_duration_since(
                DateTime::parse_from_rfc3339("2023-04-10T14:37:00Z")?,
                DateTime::parse_from_rfc3339("2023-04-10T13:37:00Z")?,
            )
        );
        Ok(())
    }

    #[test]
    fn should_use_correct_temporal_unit() -> Result<(), ParseError> {
        assert_eq!(
            RelativeTimestamp::InMinutes(Duration::minutes(42)),
            RenovateManager::get_relative_timestamp(Duration::minutes(42))
        );
        assert_eq!(
            RelativeTimestamp::InMinutes(Duration::minutes(59)),
            RenovateManager::get_relative_timestamp(Duration::minutes(59))
        );
        assert_eq!(
            RelativeTimestamp::InHours(Duration::hours(1)),
            RenovateManager::get_relative_timestamp(Duration::minutes(60))
        );
        assert_eq!(
            RelativeTimestamp::InHours(Duration::hours(8760)),
            RenovateManager::get_relative_timestamp(Duration::hours(8760))
        );
        Ok(())
    }

    #[test]
    fn should_provide_relative_timestamp() {
        assert_eq!(
            "just now",
            &RelativeTimestamp::InMinutes(Duration::seconds(5)).to_string()
        );
        assert_eq!(
            "1 minute ago",
            &RelativeTimestamp::InMinutes(Duration::minutes(1)).to_string()
        );
        assert_eq!(
            "1 minute ago",
            &RelativeTimestamp::InMinutes(Duration::seconds(100)).to_string()
        );
        assert_eq!(
            "2 minutes ago",
            &RelativeTimestamp::InMinutes(Duration::seconds(150)).to_string()
        );
        assert_eq!(
            "2 minutes ago",
            &RelativeTimestamp::InMinutes(Duration::minutes(2)).to_string()
        );
        assert_eq!(
            "50 minutes ago",
            &RelativeTimestamp::InMinutes(Duration::minutes(50)).to_string()
        );

        assert_eq!(
            "1 hour ago",
            &RelativeTimestamp::InHours(Duration::minutes(100)).to_string()
        );
        assert_eq!(
            "1 hour ago",
            &RelativeTimestamp::InHours(Duration::hours(1)).to_string()
        );
        assert_eq!(
            "2 hours ago",
            &RelativeTimestamp::InHours(Duration::minutes(150)).to_string()
        );
        assert_eq!(
            "2 hours ago",
            &RelativeTimestamp::InHours(Duration::hours(2)).to_string()
        );
        assert_eq!(
            "100 hours ago",
            &RelativeTimestamp::InHours(Duration::hours(100)).to_string()
        );
    }
}
