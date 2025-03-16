#[cfg(test)]
mod tests {
    use crate::client::task::TaskClient;
    use crate::endpoint::test_helper::setup_test_app;
    use actix_web::test;
    use chrono::DateTime;
    use chrono::Utc;
    use openapi::model::task::{ApiTask, ApiTaskPriority, ApiTaskStatus};
    use openapi::request::task::{ApiTaskCreateBody, ApiTaskUpdateBody};

    #[actix_web::test]
    async fn test_task_create() {
        let app = setup_test_app().await;

        let client = TaskClient::new();
        let create_request = ApiTaskCreateBody {
            title: "テストタスク".to_string(),
            description: "タスクの説明".to_string(),
            status: ApiTaskStatus::Todo,
            priority: ApiTaskPriority::Low,
            tags: vec![],
            deadline: Some("2012-12-12 12:12:12Z".parse::<DateTime<Utc>>().unwrap()),
        };
        client.create(&app, create_request).await;

        let list_resp = client.list(&app).await;

        assert!(list_resp.is_success());

        let list_body = list_resp.to_body::<Vec<ApiTask>>().await;

        assert_eq!(list_body.len(), 1);
        assert_eq!(list_body[0].title, "テストタスク");
        assert_eq!(list_body[0].description, "タスクの説明".to_string());
        assert_eq!(list_body[0].status, ApiTaskStatus::Todo);
        assert_eq!(list_body[0].priority, ApiTaskPriority::Low);
        assert_eq!(list_body[0].tags, vec![]);
        assert_eq!(
            list_body[0].deadline,
            Some("2012-12-12 12:12:12Z".parse::<DateTime<Utc>>().unwrap())
        );
    }

    #[actix_web::test]
    async fn test_task_get() {
        let app = setup_test_app().await;

        let client = TaskClient::new();
        let create_request = ApiTaskCreateBody {
            title: "テストタスク".to_string(),
            description: "タスクの説明".to_string(),
            status: ApiTaskStatus::Todo,
            priority: ApiTaskPriority::Low,
            tags: vec![],
            deadline: Some("2012-12-12 12:12:12Z".parse::<DateTime<Utc>>().unwrap()),
        };
        let create_resp = client.create(&app, create_request).await;

        let task_id = create_resp.to_body::<ApiTask>().await.id;
        let get_resp = client.get(&app, &task_id).await;

        assert!(get_resp.is_success());

        let get_body = get_resp.to_body::<ApiTask>().await;

        assert_eq!(get_body.title, "テストタスク");
        assert_eq!(get_body.description, "タスクの説明".to_string());
        assert_eq!(get_body.status, ApiTaskStatus::Todo);
        assert_eq!(get_body.priority, ApiTaskPriority::Low);
        assert_eq!(get_body.tags, vec![]);
        assert_eq!(
            get_body.deadline,
            Some("2012-12-12 12:12:12Z".parse::<DateTime<Utc>>().unwrap())
        );
    }

    #[actix_web::test]
    async fn test_task_update() {
        let app = setup_test_app().await;
        let client = TaskClient::new();

        // 準備：タスクを作成
        let create_request = ApiTaskCreateBody {
            title: "テストタスク".to_string(),
            description: "タスクの説明".to_string(),
            status: ApiTaskStatus::Todo,
            priority: ApiTaskPriority::Low,
            tags: vec![],
            deadline: Some("2012-12-12 12:12:12Z".parse::<DateTime<Utc>>().unwrap()),
        };
        let create_resp = client.create(&app, create_request).await;
        assert!(create_resp.is_success());
        let created_task = create_resp.to_body::<ApiTask>().await;

        // 更新を実行
        let update_request = ApiTaskUpdateBody {
            project_id: None,
            title: Some("更新後のタスク".to_string()),
            description: Some("更新後の説明".to_string()),
            status: Some(ApiTaskStatus::InProgress),
            priority: Some(ApiTaskPriority::High),
            tags: None,
            deadline: Some("2012-12-12 12:13:12Z".parse::<DateTime<Utc>>().unwrap()),
        };
        let update_resp = client.update(&app, &created_task.id, update_request).await;
        assert!(update_resp.is_success());

        let updated_task = update_resp.to_body::<ApiTask>().await;
        assert_eq!(updated_task.title, "更新後のタスク");
        assert_eq!(updated_task.description, "更新後の説明");
        assert_eq!(updated_task.status, ApiTaskStatus::InProgress);
        assert_eq!(updated_task.priority, ApiTaskPriority::High);
        assert_eq!(updated_task.tags, vec![]);
        assert_eq!(
            updated_task.deadline,
            Some("2012-12-12 12:13:12Z".parse::<DateTime<Utc>>().unwrap())
        );
    }

    #[actix_web::test]
    async fn test_task_delete() {
        let app = setup_test_app().await;
        let client = TaskClient::new();

        // 準備：タスクを作成
        let create_request = ApiTaskCreateBody {
            title: "テストタスク".to_string(),
            description: "タスクの説明".to_string(),
            status: ApiTaskStatus::Todo,
            priority: ApiTaskPriority::Low,
            tags: vec![],
            deadline: Some("2012-12-12 12:12:12Z".parse::<DateTime<Utc>>().unwrap()),
        };
        let create_resp = client.create(&app, create_request).await;
        assert!(create_resp.is_success());
        let created_task = create_resp.to_body::<ApiTask>().await;

        // 削除を実行
        let delete_resp = client.delete(&app, &created_task.id).await;
        assert!(delete_resp.is_success());

        // 削除の確認
        let get_resp = client.get(&app, &created_task.id).await;
        assert!(!get_resp.is_success());
        assert_eq!(get_resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_task_crud_flow() {
        let app = setup_test_app().await;
        let client = TaskClient::new();

        // 作成
        let create_request = ApiTaskCreateBody {
            title: "テストタスク".to_string(),
            description: "タスクの説明".to_string(),
            status: ApiTaskStatus::Todo,
            priority: ApiTaskPriority::Low,
            tags: vec![],
            deadline: Some("2012-12-12 12:12:12Z".parse::<DateTime<Utc>>().unwrap()),
        };
        let create_resp = client.create(&app, create_request).await;
        assert!(create_resp.is_success());
        let created_task = create_resp.to_body::<ApiTask>().await;

        // 取得
        let get_resp = client.get(&app, &created_task.id.clone()).await;
        assert!(get_resp.is_success());
        let get_task = get_resp.to_body::<ApiTask>().await;
        assert_eq!(get_task.id, created_task.id);

        // 更新
        let update_request = ApiTaskUpdateBody {
            project_id: None,
            title: Some("更新後のタスク".to_string()),
            description: Some("更新後の説明".to_string()),
            status: Some(ApiTaskStatus::InProgress),
            priority: Some(ApiTaskPriority::High),
            tags: None,
            deadline: Some("2012-12-12 12:13:12Z".parse::<DateTime<Utc>>().unwrap()),
        };
        let update_resp = client.update(&app, &created_task.id, update_request).await;
        assert!(update_resp.is_success());
        let updated_task = update_resp.to_body::<ApiTask>().await;
        assert_eq!(updated_task.title, "更新後のタスク");

        // 一覧取得
        let list_resp = client.list(&app).await;
        assert!(list_resp.is_success());
        let tasks = list_resp.to_body::<Vec<ApiTask>>().await;
        assert!(tasks.iter().any(|t| t.id == created_task.id));

        // 削除
        let delete_resp = client.delete(&app, &created_task.id).await;
        assert!(delete_resp.is_success());

        // 削除確認
        let get_deleted_resp = client.get(&app, &created_task.id).await;
        assert!(!get_deleted_resp.is_success());
        assert_eq!(get_deleted_resp.status(), 404);
    }
}
