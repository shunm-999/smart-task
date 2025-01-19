use crate::model::project::ApiProject;
use crate::{create_response, ok_response};
use smart_task_openapi_axum::apis::project::{
    ProjectsCreateResponse, ProjectsDeleteResponse, ProjectsGetResponse, ProjectsListResponse,
    ProjectsUpdateResponse,
};

create_response!(
    ApiProjectsCreateResponse,
    ProjectsCreateResponse,
    MapToProjectsCreateResponse,
    ApiProject,
    map_to_create_project_response
);
ok_response!(
    ApiProjectsGetResponse,
    ProjectsGetResponse,
    MapToProjectsGetResponse,
    ApiProject,
    map_to_get_project_response
);
ok_response!(
    ApiProjectsListResponse,
    ProjectsListResponse,
    MapToProjectsListResponse,
    Vec<ApiProject>,
    map_to_list_project_response
);
ok_response!(
    ApiProjectsUpdateResponse,
    ProjectsUpdateResponse,
    MapToProjectsUpdateResponse,
    ApiProject,
    map_to_update_project_response
);
ok_response!(
    ApiProjectsDeleteResponse,
    ProjectsDeleteResponse,
    MapToProjectsDeleteResponse,
    ApiProject,
    map_to_delete_project_response
);
