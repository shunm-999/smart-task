use crate::repository::SmartTaskRepository;
use domain::model::project::{Project, ProjectCreation, ProjectId, ProjectUpdating};
use domain::repository::project::ProjectRepository;

impl ProjectRepository for SmartTaskRepository {
    async fn get_projects(&self) -> domain::Result<Vec<Project>> {
        todo!()
    }

    async fn get_project(&self, project_id: ProjectId) -> domain::Result<Option<Project>> {
        todo!()
    }

    async fn create_project(&self, project_creation: ProjectCreation) -> domain::Result<Project> {
        todo!()
    }

    async fn update_project(&self, project_updating: ProjectUpdating) -> domain::Result<Project> {
        todo!()
    }

    async fn delete_project(&self, project_id: ProjectId) -> domain::Result<Project> {
        todo!()
    }
}
