use crate::database::entity::tag::Entity as TagEntity;
use crate::database::entity::tag_task::Entity as TagTaskEntity;
use crate::database::entity::task::{
    ActiveModel, Entity as TaskEntity, Priority, Status, TagTaskRelation,
};
use crate::database::error::map_to_domain_error;
use crate::repository::DatabaseRepository;
use domain::model::task::{Task, TaskCreation, TaskId, TaskPriority, TaskStatus, TaskUpdating};
use domain::repository::task::TaskRepository;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, LoaderTrait};

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
            .unwrap_or_else(|| vec![])
            .clone();

        let tasks: TagTaskRelation = (task, tags).into();
        Ok(tasks.into())
    }

    async fn create_task(&self, task_creation: TaskCreation) -> domain::Result<Task> {
        // let conn = self.database_connection_provider.get_connection();
        // let task: ActiveModel = task_creation.into();
        // let task = task
        //     .insert(conn)
        //     .await
        //     .map_err(|e| map_to_domain_error(e))?;
        // Ok(task.into())
        todo!()
    }

    async fn update_task(&self, task_updating: TaskUpdating) -> domain::Result<Task> {
        todo!()
    }

    async fn delete_task(&self, task_id: TaskId) -> domain::Result<Task> {
        todo!()
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
