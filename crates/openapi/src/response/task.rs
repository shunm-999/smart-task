use crate::{create_response, ok_response};
use smart_task_openapi_axum::apis::task::{
    TasksCreateResponse, TasksDeleteResponse, TasksGetResponse, TasksListResponse,
    TasksUpdateResponse,
};

create_response!(TasksCreateResponse);

ok_response!(TasksGetResponse);
ok_response!(TasksListResponse);
ok_response!(TasksUpdateResponse);
ok_response!(TasksDeleteResponse);
