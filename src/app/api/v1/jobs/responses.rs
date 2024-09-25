use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::storage::types::{JobDetails, JobStatus};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub project_cycle_id: Option<Uuid>,
    pub status: JobStatus,
    pub label: String,
    pub description: Option<String>,
    pub details: JobDetails,
}

// impl TryFrom<Job> for JobTypeResponse {
//     type Error = anyhow::Error;
//
//     fn try_from(job: Job) -> Result<Self, Self::Error> {
//         Ok(Self {
//             id: job.id,
//             created_at: job.created_at,
//             updated_at: job.updated_at,
//             project_cycle_id: job.project_cycle_id,
//             status: job.status,
//             label: job.label,
//             description: job.description,
//             details: serde_json::from_value::<JobDetails>(job.details)?,
//         })
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct JobsResponse {
    pub jobs: Vec<Job>,
}
