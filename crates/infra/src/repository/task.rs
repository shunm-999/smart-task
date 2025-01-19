use crate::repository::SmartTaskRepository;
use domain::model::task::{Task, TaskCreation, TaskId, TaskUpdating};
use domain::repository::task::TaskRepository;

impl TaskRepository for SmartTaskRepository {
    async fn get_tasks(&self) -> domain::Result<Vec<Task>> {
        todo!()
    }

    async fn get_task(&self, task_id: TaskId) -> domain::Result<Option<Task>> {
        todo!()
    }

    async fn create_task(&self, task_creation: TaskCreation) -> domain::Result<Task> {
        todo!()
    }

    async fn update_task(&self, task_updating: TaskUpdating) -> domain::Result<Task> {
        todo!()
    }

    async fn delete_task(&self, task_id: TaskId) -> domain::Result<Task> {
        todo!()
    }
}
