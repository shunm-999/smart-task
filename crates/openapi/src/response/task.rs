use crate::models::task::ApiTask;
use crate::{create_response, ok_response};
use smart_task_openapi_axum::apis::task::{
    TasksCreateResponse, TasksDeleteResponse, TasksGetResponse, TasksListResponse,
    TasksUpdateResponse,
};

create_response!(TasksCreateResponse, ApiTask);

ok_response!(TasksGetResponse, ApiTask);
ok_response!(TasksListResponse, Vec<ApiTask>);
ok_response!(TasksUpdateResponse, ApiTask);
ok_response!(TasksDeleteResponse, ApiTask);
