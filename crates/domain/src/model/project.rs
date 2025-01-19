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
