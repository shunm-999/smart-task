use crate::{create_response, ok_response};
use smart_task_openapi_axum::apis::project::{
    ProjectsCreateResponse, ProjectsDeleteResponse, ProjectsGetResponse, ProjectsListResponse,
    ProjectsUpdateResponse,
};

create_response!(ProjectsCreateResponse);
ok_response!(ProjectsGetResponse);
ok_response!(ProjectsListResponse);
ok_response!(ProjectsUpdateResponse);
ok_response!(ProjectsDeleteResponse);
