use crate::model::project::{Project, ProjectCreation, ProjectId, ProjectUpdating};
use crate::Result;

pub trait ProjectRepository {
    async fn get_projects(&self) -> Result<Vec<Project>>;
    async fn get_project(&self, project_id: ProjectId) -> Result<Project>;
    async fn create_project(&self, project_creation: ProjectCreation) -> Result<Project>;
    async fn update_project(&self, project_updating: ProjectUpdating) -> Result<Project>;
    async fn delete_project(&self, project_id: ProjectId) -> Result<Project>;
}
