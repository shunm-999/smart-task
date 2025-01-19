use crate::model::task::{Task, TaskCreation, TaskId, TaskUpdating};
use crate::Result;

pub trait TaskRepository {
    fn get_tasks(&self) -> Result<Vec<Task>>;
    fn get_task(&self, task_id: TaskId) -> Result<Option<Task>>;
    fn create_task(&self, task_creation: TaskCreation) -> Result<Task>;
    fn update_task(&self, task_updating: TaskUpdating) -> Result<Task>;
    fn delete_task(&self, task_id: TaskId) -> Result<Task>;
}
