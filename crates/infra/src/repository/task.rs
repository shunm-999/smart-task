use crate::database::entity::tag::Entity as TagEntity;
use crate::database::entity::tag_task;
use crate::database::entity::tag_task::Entity as TagTaskEntity;
use crate::database::entity::task::{
    ActiveModel, Entity as TaskEntity, Model, Priority, Status, TagTaskRelation,
};
use crate::database::error::map_db_error_to_domain_error;
use crate::SmartTaskRepositoryImpl;
use domain::model::tag::TagId;
use domain::model::task::{Task, TaskCreation, TaskId, TaskPriority, TaskStatus, TaskUpdating};
use domain::repository::task::TaskRepository;
use domain::Error;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, DatabaseTransaction, EntityTrait, LoaderTrait, NotSet, TransactionTrait,
};

impl TaskRepository for SmartTaskRepositoryImpl {
    async fn get_tasks(&self) -> domain::Result<Vec<Task>> {
        let tasks = self
            .database_connection_provider
            .transaction::<_, Vec<Task>>(|txn| {
                Box::pin(async move {
                    let tasks: Vec<TagTaskRelation> = get_tag_task_relation(txn).await?;
                    let tasks = tasks.into_iter().map(|task| task.into()).collect();
                    Ok(tasks)
                })
            })
            .await?;
        Ok(tasks)
    }

    async fn get_task(&self, task_id: TaskId) -> domain::Result<Task> {
        let task = self
            .database_connection_provider
            .transaction::<_, Task>(|txn| {
                Box::pin(async move {
                    let task: TagTaskRelation = get_tag_task_relation_one(txn, task_id).await?;
                    Ok(task.into())
                })
            })
            .await?;
        Ok(task)
    }

    async fn create_task(&self, task_creation: TaskCreation) -> domain::Result<Task> {
        let task = self
            .database_connection_provider
            .transaction::<_, Task>(|txn| {
                Box::pin(async move {
                    let task: ActiveModel = task_creation.clone().into();
                    task.insert(txn)
                        .await
                        .map_err(|e| map_db_error_to_domain_error(e))?;
                    let task: TagTaskRelation =
                        get_tag_task_relation_one(txn, task_creation.id).await?;
                    Ok(task.into())
                })
            })
            .await?;
        Ok(task)
    }

    async fn update_task(&self, task_updating: TaskUpdating) -> domain::Result<Task> {
        let task = self
            .database_connection_provider
            .transaction::<_, Task>(|txn| {
                Box::pin(async move {
                    let task: TagTaskRelation =
                        get_tag_task_relation_one(txn, task_updating.id).await?;
                    let task: ActiveModel = (task.0, task_updating.clone()).into();
                    task.update(txn)
                        .await
                        .map_err(|e| map_db_error_to_domain_error(e))?;

                    if let Some(update_tags) = task_updating.tags {
                        TagTaskEntity::delete_many()
                            .filter(tag_task::Column::TaskId.eq(task_updating.id.to_string()))
                            .exec(txn)
                            .await
                            .map_err(|e| map_db_error_to_domain_error(e))?;

                        let update_tags: Vec<tag_task::ActiveModel> = update_tags
                            .into_iter()
                            .map(|tag| (task_updating.id, tag.id).into())
                            .collect();
                        TagTaskEntity::insert_many(update_tags)
                            .exec(txn)
                            .await
                            .map_err(|e| map_db_error_to_domain_error(e))?;
                    }

                    let tasks: TagTaskRelation =
                        get_tag_task_relation_one(txn, task_updating.id).await?;
                    Ok(tasks.into())
                })
            })
            .await?;
        Ok(task)
    }

    async fn delete_task(&self, task_id: TaskId) -> domain::Result<Task> {
        let task = self.get_task(task_id.clone()).await?;
        TaskEntity::delete_by_id(*task_id.as_ref())
            .exec(self.database_connection_provider.get_connection())
            .await
            .map_err(|e| map_db_error_to_domain_error(e))?;
        Ok(task)
    }
}

async fn get_tag_task_relation(txn: &DatabaseTransaction) -> domain::Result<Vec<TagTaskRelation>> {
    let tasks = TaskEntity::find()
        .all(txn)
        .await
        .map_err(|e| map_db_error_to_domain_error(e))?;
    let tags = tasks
        .load_many_to_many(TagEntity, TagTaskEntity, txn)
        .await
        .map_err(|e| map_db_error_to_domain_error(e))?;

    let tasks = tasks
        .into_iter()
        .zip(tags)
        .map(|(task, tags)| (task, tags).into())
        .collect();
    Ok(tasks)
}

async fn get_tag_task_relation_one(
    txn: &DatabaseTransaction,
    task_id: TaskId,
) -> domain::Result<TagTaskRelation> {
    let task = TaskEntity::find_by_id(*task_id.as_ref())
        .one(txn)
        .await
        .map_err(|e| map_db_error_to_domain_error(e))?
        .ok_or(Error::NotFound)?;

    let tags = vec![task.clone()]
        .load_many_to_many(TagEntity, TagTaskEntity, txn)
        .await
        .map_err(|e| map_db_error_to_domain_error(e))?
        .first()
        .cloned()
        .unwrap_or_else(|| vec![]);

    Ok((task, tags).into())
}

impl From<TaskStatus> for Status {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::Todo => Self::Todo,
            TaskStatus::InProgress => Self::InProgress,
            TaskStatus::Done => Self::Done,
        }
    }
}

impl From<TaskPriority> for Priority {
    fn from(value: TaskPriority) -> Self {
        match value {
            TaskPriority::Low => Self::Low,
            TaskPriority::Medium => Self::Medium,
            TaskPriority::High => Self::High,
        }
    }
}

impl From<TaskCreation> for ActiveModel {
    fn from(value: TaskCreation) -> Self {
        let now = chrono::Utc::now().naive_utc();
        ActiveModel {
            id: Set(*value.id.as_ref()),
            project_id: Set(value.project_id.map(|id| *id.as_ref())),
            title: Set(value.title),
            description: Set(value.description),
            status: Set(value.status.into()),
            priority: Set(value.priority.into()),
            deadline: Set(value.deadline.map(|d| d.naive_utc())),
            created_at: Set(now),
            updated_at: Set(now),
        }
    }
}

impl From<(Model, TaskUpdating)> for ActiveModel {
    fn from((task, updating): (Model, TaskUpdating)) -> Self {
        let mut task: ActiveModel = task.into();

        if let Some(project_id) = updating.project_id {
            task.project_id = match project_id {
                None => NotSet,
                Some(project_id) => Set(Some(*project_id.as_ref())),
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
            task_id: Set(*task_id.as_ref()),
            tag_id: Set(*tag_id.as_ref()),
        }
    }
}
