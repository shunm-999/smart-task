use crate::dto::{MapToApiModel, MapToDomain};
use domain::model::task::{Task, TaskCreation, TaskId, TaskPriority, TaskStatus, TaskUpdating};
use openapi::model::task::{ApiTask, ApiTaskPriority, ApiTaskStatus};
use openapi::request::task::{ApiTaskCreateBody, ApiTaskUpdateBody};

impl MapToApiModel<ApiTask> for Task {
    fn map_to_api_model(self) -> ApiTask {
        ApiTask {
            id: self.id.to_string(),
            project_id: self.project_id.map(|pid| pid.to_string()),
            title: self.title,
            description: self.description,
            status: self.status.map_to_api_model(),
            priority: self.priority.map_to_api_model(),
            tags: self
                .tags
                .into_iter()
                .map(|t| t.map_to_api_model())
                .collect(),
            deadline: self.deadline,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl MapToApiModel<ApiTaskStatus> for TaskStatus {
    fn map_to_api_model(self) -> ApiTaskStatus {
        match self {
            TaskStatus::Todo => ApiTaskStatus::Todo,
            TaskStatus::InProgress => ApiTaskStatus::InProgress,
            TaskStatus::Done => ApiTaskStatus::Done,
        }
    }
}

impl MapToApiModel<ApiTaskPriority> for TaskPriority {
    fn map_to_api_model(self) -> ApiTaskPriority {
        match self {
            TaskPriority::High => ApiTaskPriority::High,
            TaskPriority::Medium => ApiTaskPriority::Medium,
            TaskPriority::Low => ApiTaskPriority::Low,
        }
    }
}

impl MapToDomain<TaskCreation> for ApiTaskCreateBody {
    fn map_to_domain(self) -> TaskCreation {
        TaskCreation {
            id: TaskId::new(),
            project_id: None,
            title: self.title,
            description: self.description,
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            tags: vec![],
            deadline: self.deadline,
        }
    }
}

impl MapToDomain<TaskUpdating> for (String, ApiTaskUpdateBody) {
    fn map_to_domain(self) -> TaskUpdating {
        let (id, body) = self;
        TaskUpdating {
            id: (&id).into(),
            project_id: None,
            title: body.title,
            description: body.description,
            status: body.status.map(|s| s.map_to_domain()),
            priority: body.priority.map(|p| p.map_to_domain()),
            tags: body
                .tags
                .map(|tags| tags.into_iter().map(|t| t.map_to_domain()).collect()),
            deadline: body.deadline.map(|d| Some(d)),
        }
    }
}

impl MapToDomain<TaskStatus> for ApiTaskStatus {
    fn map_to_domain(self) -> TaskStatus {
        match self {
            ApiTaskStatus::Todo => TaskStatus::Todo,
            ApiTaskStatus::InProgress => TaskStatus::InProgress,
            ApiTaskStatus::Done => TaskStatus::Done,
        }
    }
}

impl MapToDomain<TaskPriority> for ApiTaskPriority {
    fn map_to_domain(self) -> TaskPriority {
        match self {
            ApiTaskPriority::High => TaskPriority::High,
            ApiTaskPriority::Medium => TaskPriority::Medium,
            ApiTaskPriority::Low => TaskPriority::Low,
        }
    }
}
