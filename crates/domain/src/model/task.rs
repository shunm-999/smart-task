use crate::model::project::ProjectId;
use crate::model::tag::Tag;
use crate::{data_enum, data_id, data_model};

data_id!(TaskId);

data_enum!(TaskPriority, High, Middle, Low);

data_enum!(TaskStatus, Inbox, NextAction, Waiting, Done);

data_model!(
    Task,
    id: TaskId,
    project_id: Option<ProjectId>,
    title: String,
    description: String,
    status: TaskStatus,
    priority: TaskPriority,
    tags: Vec<Tag>,
    deadline: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>
);
