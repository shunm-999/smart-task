use crate::database::entity::tag::Entity as TagEntity;
use crate::database::entity::tag_task;
use crate::database::entity::tag_task::Entity as TagTaskEntity;
use crate::database::entity::task::{
    ActiveModel, Entity as TaskEntity, Model, Priority, Status, TagTaskRelation,
};
use crate::database::error::map_to_domain_error;
use crate::repository::DatabaseRepository;
use domain::model::tag::TagId;
use domain::model::task::{Task, TaskCreation, TaskId, TaskPriority, TaskStatus, TaskUpdating};
use domain::repository::task::TaskRepository;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, LoaderTrait, NotSet};

impl TaskRepository for DatabaseRepository {
    async fn get_tasks(&self) -> domain::Result<Vec<Task>> {
        let conn = self.database_connection_provider.get_connection();
        let tasks = TaskEntity::find()
            .all(conn)
            .await
            .map_err(|e| map_to_domain_error(e))?;
        let tags = tasks
            .load_many_to_many(TagEntity, TagTaskEntity, conn)
            .await
            .map_err(|e| map_to_domain_error(e))?;

        let tasks: Vec<TagTaskRelation> = tasks
            .into_iter()
            .zip(tags)
            .map(|(task, tags)| (task, tags).into())
            .collect();
        tasks.into_iter().map(|task| Ok(task.into())).collect()
    }

    async fn get_task(&self, task_id: TaskId) -> domain::Result<Task> {
        let conn = self.database_connection_provider.get_connection();
        let tasks: TagTaskRelation = self.get_tag_task_relation_one(conn, task_id).await?;
        Ok(tasks.into())
    }

    async fn create_task(&self, task_creation: TaskCreation) -> domain::Result<Task> {
        let conn = self.database_connection_provider.get_connection();
        let task: ActiveModel = task_creation.clone().into();
        task.insert(conn)
            .await
            .map_err(|e| map_to_domain_error(e))?;
        let task: TagTaskRelation = self
            .get_tag_task_relation_one(conn, task_creation.id)
            .await?;
        Ok(task.into())
    }

    async fn update_task(&self, task_updating: TaskUpdating) -> domain::Result<Task> {
        let conn = self.database_connection_provider.get_connection();
        let task = TaskEntity::find_by_id(task_updating.clone().id.to_string())
            .one(conn)
            .await
            .map_err(|e| map_to_domain_error(e))?
            .ok_or(domain::Error::NotFound)?;

        let task: ActiveModel = (task, task_updating.clone()).into();
        task.update(conn)
            .await
            .map_err(|e| map_to_domain_error(e))?;

        if let Some(update_tags) = task_updating.tags {
            TagTaskEntity::delete_many()
                .filter(tag_task::Column::TaskId.eq(task_updating.id.to_string()))
                .exec(conn)
                .await
                .map_err(|e| map_to_domain_error(e))?;

            let update_tags: Vec<tag_task::ActiveModel> = update_tags
                .into_iter()
                .map(|tag| (task_updating.id, tag.id).into())
                .collect();
            TagTaskEntity::insert_many(update_tags)
                .exec(conn)
                .await
                .map_err(|e| map_to_domain_error(e))?;
        }

        let tasks: TagTaskRelation = self.get_tag_task_relation_one(conn, task_updating.id).await?;
        Ok(tasks.into())
    }

    async fn delete_task(&self, task_id: TaskId) -> domain::Result<Task> {
        todo!()
    }
}

impl DatabaseRepository {
    async fn get_tag_task_relation_one(
        &self,
        conn: &DatabaseConnection,
        task_id: TaskId,
    ) -> domain::Result<TagTaskRelation> {
        let task = TaskEntity::find_by_id(task_id.to_string())
            .one(conn)
            .await
            .map_err(|e| map_to_domain_error(e))?
            .ok_or(domain::Error::NotFound)?;

        let tags = vec![task.clone()]
            .load_many_to_many(TagEntity, TagTaskEntity, conn)
            .await
            .map_err(|e| map_to_domain_error(e))?
            .first()
            .cloned()
            .unwrap_or_else(|| vec![]);

        Ok((task, tags).into())
    }
}

impl From<TaskStatus> for Status {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::Inbox => Self::Inbox,
            TaskStatus::NextAction => Self::NextAction,
            TaskStatus::Waiting => Self::Waiting,
            TaskStatus::Done => Self::Done,
        }
    }
}

impl From<TaskPriority> for Priority {
    fn from(value: TaskPriority) -> Self {
        match value {
            TaskPriority::Low => Self::Low,
            TaskPriority::Middle => Self::Medium,
            TaskPriority::High => Self::High,
        }
    }
}

impl From<TaskCreation> for ActiveModel {
    fn from(value: TaskCreation) -> Self {
        ActiveModel {
            id: Set(value.id.to_string()),
            project_id: Set(value.project_id.map(|id| id.to_string())),
            title: Set(value.title),
            description: Set(value.description),
            status: Set(value.status.into()),
            priority: Set(value.priority.into()),
            deadline: Set(value.deadline.map(|d| d.naive_utc())),
            ..Default::default()
        }
    }
}

impl From<(Model, TaskUpdating)> for ActiveModel {
    fn from((task, updating): (Model, TaskUpdating)) -> Self {
        let mut task: ActiveModel = task.into();

        if let Some(project_id) = updating.project_id {
            task.project_id = match project_id {
                None => NotSet,
                Some(project_id) => Set(Some(project_id.to_string())),
            };
        }
        if let Some(title) = updating.title {
            task.title = Set(title);
        }
        if let Some(description) = updating.description {
            task.description = Set(description);
        }
        if let Some(status) = updating.status {
            task.status = Set(status.into());
        }
        if let Some(priority) = updating.priority {
            task.priority = Set(priority.into());
        }
        if let Some(deadline) = updating.deadline {
            task.deadline = match deadline {
                None => NotSet,
                Some(deadline) => Set(Some(deadline.naive_utc())),
            };
        }
        task.updated_at = Set(chrono::Utc::now().naive_utc());
        task
    }
}

impl From<(TaskId, TagId)> for tag_task::ActiveModel {
    fn from((task_id, tag_id): (TaskId, TagId)) -> Self {
        Self {
            task_id: Set(task_id.to_string()),
            tag_id: Set(tag_id.to_string()),
        }
    }
}
