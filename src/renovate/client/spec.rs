use serde::{de::Visitor, Deserialize};

#[derive(Debug, PartialEq, Eq)]
pub enum JobResult {
    Done,
    Other(String),
}

struct JobResultVisitor;

impl<'de> Visitor<'de> for JobResultVisitor {
    type Value = JobResult;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "DONE" => Ok(JobResult::Done),
            _ => Ok(JobResult::Other(v.to_owned())),
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.as_str() {
            "DONE" => Ok(JobResult::Done),
            _ => Ok(JobResult::Other(v)),
        }
    }
}

impl<'de> Deserialize<'de> for JobResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(JobResultVisitor)
    }
}

#[derive(Deserialize, Debug)]
pub struct Job {
    #[serde(rename(deserialize = "finishedAt"))]
    pub finished_at: Option<String>, // RFC3339 datestring
    pub id: String, // UUID
    pub result: JobResult,
}

#[derive(Deserialize, Debug)]
pub struct JobsResponse {
    #[serde(rename(deserialize = "recentJobs"))]
    pub recent_jobs: Vec<Job>,
}
