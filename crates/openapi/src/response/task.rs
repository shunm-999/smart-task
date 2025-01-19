use crate::model::task::ApiTask;
use crate::{create_response, ok_response};
use smart_task_openapi_axum::apis::task::{
    TasksCreateResponse, TasksDeleteResponse, TasksGetResponse, TasksListResponse,
    TasksUpdateResponse,
};

create_response!(
    ApiTasksCreateResponse,
    TasksCreateResponse,
    MapToTasksCreateResponse,
    ApiTask,
    map_to_create_task_response
);

ok_response!(
    ApiTasksGetResponse,
    TasksGetResponse,
    MapToTasksGetResponse,
    ApiTask,
    map_to_get_task_response
);
ok_response!(
    ApiTasksListResponse,
    TasksListResponse,
    MapToTasksListResponse,
    Vec<ApiTask>,
    map_to_list_task_response
);
ok_response!(
    ApiTasksUpdateResponse,
    TasksUpdateResponse,
    MapToTasksUpdateResponse,
    ApiTask,
    map_to_update_task_response
);
ok_response!(
    ApiTasksDeleteResponse,
    TasksDeleteResponse,
    MapToTasksDeleteResponse,
    ApiTask,
    map_to_delete_task_response
);
