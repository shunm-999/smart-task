use crate::model::project::{Project, ProjectCreation, ProjectId, ProjectUpdating};
use crate::Result;

pub trait ProjectRepository {
    fn get_projects(&self) -> Result<Vec<Project>>;
    fn get_project(&self, project_id: ProjectId) -> Result<Option<Project>>;
    fn create_project(&self, project_creation: ProjectCreation) -> Result<Project>;
    fn update_project(&self, project_updating: ProjectUpdating) -> Result<Project>;
    fn delete_project(&self, project_id: ProjectId) -> Result<Project>;
}
