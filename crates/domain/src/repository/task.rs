use crate::model::task::{Task, TaskCreation, TaskId, TaskUpdating};
use crate::Result;

pub trait TaskRepository {
    async fn get_tasks(&self) -> Result<Vec<Task>>;
    async fn get_task(&self, task_id: TaskId) -> Result<Task>;
    async fn create_task(&self, task_creation: TaskCreation) -> Result<Task>;
    async fn update_task(&self, task_updating: TaskUpdating) -> Result<Task>;
    async fn delete_task(&self, task_id: TaskId) -> Result<Task>;
}
