use crate::dto::{MapToApiModel, MapToDomain};
use domain::model::project::{Project, ProjectCreation, ProjectId, ProjectUpdating};
use openapi::model::project::ApiProject;
use openapi::request::project::{ApiProjectCreateBody, ApiProjectUpdateBody};

impl MapToApiModel<ApiProject> for Project {
    fn map_to_api_model(self) -> ApiProject {
        ApiProject {
            id: self.id.to_string(),
            name: self.name,
            description: self.description,
            tasks: vec![], // TODO API定義から消す
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl MapToDomain<Project> for ApiProject {
    fn map_to_domain(self) -> Project {
        Project {
            id: (&self.id).into(),
            name: self.name,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl MapToDomain<ProjectCreation> for ApiProjectCreateBody {
    fn map_to_domain(self) -> ProjectCreation {
        ProjectCreation {
            id: ProjectId::new(),
            name: self.name,
            description: self.description,
        }
    }
}

impl MapToDomain<ProjectUpdating> for (String, ApiProjectUpdateBody) {
    fn map_to_domain(self) -> ProjectUpdating {
        let (id, body) = self;
        ProjectUpdating {
            id: (&id).into(),
            name: body.name,
            description: body.description,
        }
    }
}
