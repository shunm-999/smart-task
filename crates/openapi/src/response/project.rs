use crate::models::project::ApiProject;
use crate::{create_response, ok_response};
use smart_task_openapi_axum::apis::project::{
    ProjectsCreateResponse, ProjectsDeleteResponse, ProjectsGetResponse, ProjectsListResponse,
    ProjectsUpdateResponse,
};

create_response!(ProjectsCreateResponse, ApiProject);
ok_response!(ProjectsGetResponse, ApiProject);
ok_response!(ProjectsListResponse, Vec<ApiProject>);
ok_response!(ProjectsUpdateResponse, ApiProject);
ok_response!(ProjectsDeleteResponse, ApiProject);
