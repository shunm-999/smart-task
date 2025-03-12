use crate::database::entity::project::{ActiveModel, Entity as ProjectEntity, Model};
use crate::database::error::map_db_error_to_domain_error;
use crate::SmartTaskRepositoryImpl;
use domain::model::project::{Project, ProjectCreation, ProjectId, ProjectUpdating};
use domain::repository::project::ProjectRepository;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait};

impl ProjectRepository for SmartTaskRepositoryImpl {
    async fn get_projects(&self) -> domain::Result<Vec<Project>> {
        let conn = self.database_connection_provider.get_connection();
        let projects = ProjectEntity::find()
            .all(conn)
            .await
            .map_err(|e| map_db_error_to_domain_error(e))?;

        projects
            .into_iter()
            .map(|project| Ok(project.into()))
            .collect()
    }

    async fn get_project(&self, project_id: ProjectId) -> domain::Result<Project> {
        let conn = self.database_connection_provider.get_connection();
        let project = ProjectEntity::find_by_id(*project_id.as_ref())
            .one(conn)
            .await
            .map_err(|e| map_db_error_to_domain_error(e))?
            .ok_or(domain::Error::NotFound)?;

        Ok(project.into())
    }

    async fn create_project(&self, project_creation: ProjectCreation) -> domain::Result<Project> {
        let conn = self.database_connection_provider.get_connection();
        let project: ActiveModel = project_creation.into();
        let project = project
            .insert(conn)
            .await
            .map_err(|e| map_db_error_to_domain_error(e))?;

        Ok(project.into())
    }

    async fn update_project(&self, project_updating: ProjectUpdating) -> domain::Result<Project> {
        let conn = self.database_connection_provider.get_connection();
        let project = ProjectEntity::find_by_id(*project_updating.id.as_ref())
            .one(conn)
            .await
            .map_err(|e| map_db_error_to_domain_error(e))?
            .ok_or(domain::Error::NotFound)?;

        let project: ActiveModel = (project, project_updating).into();
        let project = project
            .update(conn)
            .await
            .map_err(|e| map_db_error_to_domain_error(e))?;

        Ok(project.into())
    }

    async fn delete_project(&self, project_id: ProjectId) -> domain::Result<Project> {
        let conn = self.database_connection_provider.get_connection();
        let project = ProjectEntity::find_by_id(*project_id.as_ref())
            .one(conn)
            .await
            .map_err(|e| map_db_error_to_domain_error(e))?
            .ok_or(domain::Error::NotFound)?;

        project
            .clone()
            .delete(conn)
            .await
            .map_err(|e| map_db_error_to_domain_error(e))?;

        Ok(project.into())
    }
}

impl From<ProjectCreation> for ActiveModel {
    fn from(value: ProjectCreation) -> Self {
        ActiveModel {
            id: Set(*value.id.as_ref()),
            name: Set(value.name),
            description: Set(value.description),
            ..Default::default()
        }
    }
}

impl From<(Model, ProjectUpdating)> for ActiveModel {
    fn from((project, project_updating): (Model, ProjectUpdating)) -> Self {
        let mut project: ActiveModel = project.into();

        if let Some(name) = project_updating.name {
            project.name = Set(name);
        }
        if let Some(description) = project_updating.description {
            project.description = Set(description);
        }
        project.updated_at = Set(chrono::Utc::now().naive_utc());
        project
    }
}
