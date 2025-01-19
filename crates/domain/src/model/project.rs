use crate::{data_id, data_model};

data_id!(ProjectId);

data_model!(
    Project,
    id: ProjectId,
    name: String,
    description: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>
);

pub struct ProjectCreation {
    pub name: String,
    pub description: String,
}

pub struct ProjectUpdating {
    pub id: ProjectId,
    pub name: Option<String>,
    pub description: Option<String>,
}
