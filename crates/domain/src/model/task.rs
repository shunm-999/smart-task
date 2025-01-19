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

pub struct TaskCreation {
    pub project_id: Option<ProjectId>,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub tags: Vec<Tag>,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct TaskUpdating {
    pub id: TaskId,
    pub project_id: Option<ProjectId>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub tags: Option<Vec<Tag>>,
    pub deadline: Option<Option<chrono::DateTime<chrono::Utc>>>,
}
