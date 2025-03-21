#[cfg(test)]
mod tests {
    use crate::client::project::ProjectClient;
    use crate::endpoint::test_helper::setup_test_app;
    use actix_web::test;
    use openapi::model::project::ApiProject;
    use openapi::request::project::{ApiProjectCreateBody, ApiProjectUpdateBody};

    #[actix_web::test]
    async fn test_create_project() {
        let app = setup_test_app().await;

        let client = ProjectClient::new();
        let create_request = ApiProjectCreateBody {
            name: "テストプロジェクト".to_string(),
            description: "プロジェクトの説明".to_string(),
        };
        client.create(&app, create_request).await;

        let list_resp = client.list(&app).await;

        assert!(list_resp.is_success());

        let list_body = list_resp.to_body::<Vec<ApiProject>>().await;

        assert_eq!(list_body.len(), 1);
        assert_eq!(list_body[0].name, "テストプロジェクト");
        assert_eq!(list_body[0].description, "プロジェクトの説明".to_string());
    }

    #[actix_web::test]
    async fn test_get_project() {
        let app = setup_test_app().await;

        let client = ProjectClient::new();
        let create_request = ApiProjectCreateBody {
            name: "テストプロジェクト".to_string(),
            description: "プロジェクトの説明".to_string(),
        };
        let create_resp = client.create(&app, create_request).await;

        let project_id = create_resp.to_body::<ApiProject>().await.id;
        let get_resp = client.get(&app, &project_id).await;

        assert!(get_resp.is_success());

        let get_body = get_resp.to_body::<ApiProject>().await;

        assert_eq!(get_body.name, "テストプロジェクト");
        assert_eq!(get_body.description, "プロジェクトの説明".to_string());
    }

    #[actix_web::test]
    async fn test_update_project() {
        let app = setup_test_app().await;
        let client = ProjectClient::new();

        let create_request = ApiProjectCreateBody {
            name: "テストプロジェクト".to_string(),
            description: "プロジェクトの説明".to_string(),
        };
        let create_resp = client.create(&app, create_request).await;

        let project_id = create_resp.to_body::<ApiProject>().await.id;

        let update_request = ApiProjectUpdateBody {
            name: Some("更新されたプロジェクト".to_string()),
            description: Some("更新されたプロジェクトの説明".to_string()),
        };
        let update_resp = client.update(&app, &project_id, update_request).await;

        assert!(update_resp.is_success());

        let get_resp = client.get(&app, &project_id).await;

        assert!(get_resp.is_success());

        let get_body = get_resp.to_body::<ApiProject>().await;

        assert_eq!(get_body.name, "更新されたプロジェクト");
        assert_eq!(
            get_body.description,
            "更新されたプロジェクトの説明".to_string()
        );
    }

    #[actix_web::test]
    async fn test_not_update_project_if_null_body() {
        let app = setup_test_app().await;
        let client = ProjectClient::new();

        let create_request = ApiProjectCreateBody {
            name: "テストプロジェクト".to_string(),
            description: "プロジェクトの説明".to_string(),
        };
        let create_resp = client.create(&app, create_request).await;
        let created_project = create_resp.to_body::<ApiProject>().await;

        let update_request = ApiProjectUpdateBody {
            name: None,
            description: None,
        };
        let update_resp = client
            .update(&app, &created_project.id, update_request)
            .await;
        assert!(update_resp.is_success());

        let get_resp = client.get(&app, &created_project.id).await;

        assert!(get_resp.is_success());

        let get_body = get_resp.to_body::<ApiProject>().await;

        assert_eq!(get_body.name, "テストプロジェクト");
        assert_eq!(get_body.description, "プロジェクトの説明".to_string());
    }

    #[actix_web::test]
    async fn test_delete_project() {
        let app = setup_test_app().await;
        let client = ProjectClient::new();

        let create_request = ApiProjectCreateBody {
            name: "テストプロジェクト".to_string(),
            description: "プロジェクトの説明".to_string(),
        };
        let create_resp = client.create(&app, create_request).await;

        let project_id = create_resp.to_body::<ApiProject>().await.id;

        let delete_resp = client.delete(&app, &project_id).await;

        assert!(delete_resp.is_success());

        let get_resp = client.get(&app, &project_id).await;

        assert!(!get_resp.is_success());
        assert_eq!(get_resp.status(), 404)
    }
    #[actix_web::test]
    async fn test_project_crud_flow() {
        let app = setup_test_app().await;
        let client = ProjectClient::new();

        // 作成
        let create_request = ApiProjectCreateBody {
            name: "テストプロジェクト".to_string(),
            description: "プロジェクトの説明".to_string(),
        };
        let create_resp = client.create(&app, create_request).await;
        assert!(create_resp.is_success());
        let created_project = create_resp.to_body::<ApiProject>().await;

        // 取得
        let get_resp = client.get(&app, &created_project.id.clone()).await;
        assert!(get_resp.is_success());
        let get_project = get_resp.to_body::<ApiProject>().await;
        assert_eq!(get_project.id, created_project.id);

        // 更新
        let update_request = ApiProjectUpdateBody {
            name: Some("更新後のプロジェクト".to_string()),
            description: Some("更新後のプロジェクトの説明".to_string()),
        };
        let update_resp = client
            .update(&app, &created_project.id, update_request)
            .await;
        assert!(update_resp.is_success());
        let updated_project = update_resp.to_body::<ApiProject>().await;
        assert_eq!(updated_project.name, "更新後のプロジェクト");
        assert_eq!(updated_project.description, "更新後のプロジェクトの説明");

        // 削除
        let delete_resp = client.delete(&app, &created_project.id).await;
        assert!(delete_resp.is_success());

        // 削除の確認
        let get_resp = client.get(&app, &created_project.id).await;
        assert!(!get_resp.is_success());
        assert_eq!(get_resp.status(), 404);
    }
}
