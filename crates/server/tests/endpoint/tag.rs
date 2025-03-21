#[cfg(test)]
mod tests {
    use crate::client::tag::TagClient;
    use crate::endpoint::test_helper::setup_test_app;
    use actix_web::test;
    use openapi::model::tag::ApiTag;
    use openapi::request::tag::{ApiTagCreateBody, ApiTagUpdateBody};

    #[actix_web::test]
    async fn test_create_tag() {
        let app = setup_test_app().await;

        let client = TagClient::new();
        let create_request = openapi::request::tag::ApiTagCreateBody {
            name: "テストタグ".to_string(),
            color: Some(16711680),
        };
        client.create(&app, create_request).await;

        let list_resp = client.list(&app).await;

        assert!(list_resp.is_success());

        let list_body = list_resp.to_body::<Vec<ApiTag>>().await;

        assert_eq!(list_body.len(), 1);
        assert_eq!(list_body[0].name, "テストタグ");
        assert_eq!(list_body[0].color, 16711680);
    }

    #[actix_web::test]
    async fn test_get_tag() {
        let app = setup_test_app().await;

        let client = TagClient::new();
        let create_request = openapi::request::tag::ApiTagCreateBody {
            name: "テストタグ".to_string(),
            color: Some(16711680),
        };
        let create_resp = client.create(&app, create_request).await;

        let tag_id = create_resp.to_body::<ApiTag>().await.id;
        let get_resp = client.get(&app, &tag_id).await;

        assert!(get_resp.is_success());

        let get_body = get_resp.to_body::<ApiTag>().await;

        assert_eq!(get_body.name, "テストタグ");
        assert_eq!(get_body.color, 16711680);
    }

    #[actix_web::test]
    async fn test_update_tag() {
        let app = setup_test_app().await;
        let client = TagClient::new();

        // 準備：タグを作成
        let create_request = ApiTagCreateBody {
            name: "テストタグ".to_string(),
            color: Some(16711680),
        };
        let create_resp = client.create(&app, create_request).await;
        assert!(create_resp.is_success());
        let created_tag = create_resp.to_body::<ApiTag>().await;

        // 更新を実行
        let update_request = ApiTagUpdateBody {
            name: Some("更新後のタグ".to_string()),
            color: Some(255),
        };
        let update_resp = client.update(&app, &created_tag.id, update_request).await;
        assert!(update_resp.is_success());

        let updated_tag = update_resp.to_body::<ApiTag>().await;
        assert_eq!(updated_tag.name, "更新後のタグ");
        assert_eq!(updated_tag.color, 255);
        assert_eq!(updated_tag.created_at, created_tag.created_at);
        assert!(updated_tag.updated_at > created_tag.updated_at);
    }

    #[actix_web::test]
    async fn test_not_update_tag_if_null_body() {
        let app = setup_test_app().await;
        let client = TagClient::new();

        // 準備：タグを作成
        let create_request = ApiTagCreateBody {
            name: "テストタグ".to_string(),
            color: Some(16711680),
        };
        let create_resp = client.create(&app, create_request).await;
        assert!(create_resp.is_success());
        let created_tag = create_resp.to_body::<ApiTag>().await;

        // 更新を実行
        let update_request = ApiTagUpdateBody {
            name: None,
            color: None,
        };
        let update_resp = client.update(&app, &created_tag.id, update_request).await;
        assert!(update_resp.is_success());

        let updated_tag = update_resp.to_body::<ApiTag>().await;
        assert_eq!(updated_tag.name, "テストタグ");
        assert_eq!(updated_tag.color, 16711680);
    }

    #[actix_web::test]
    async fn test_delete_tag() {
        let app = setup_test_app().await;
        let client = TagClient::new();

        // 準備：タグを作成
        let create_request = ApiTagCreateBody {
            name: "テストタグ".to_string(),
            color: Some(16711680),
        };
        let create_resp = client.create(&app, create_request).await;
        assert!(create_resp.is_success());
        let created_tag = create_resp.to_body::<ApiTag>().await;

        // 削除を実行
        let delete_resp = client.delete(&app, &created_tag.id).await;
        assert!(delete_resp.is_success());

        // 削除の確認
        let get_resp = client.get(&app, &created_tag.id).await;
        assert!(!get_resp.is_success());
        assert_eq!(get_resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_crud_tag() {
        let app = setup_test_app().await;
        let client = TagClient::new();

        // 作成
        let create_request = ApiTagCreateBody {
            name: "テストタグ".to_string(),
            color: Some(16711680),
        };
        let create_resp = client.create(&app, create_request).await;
        assert!(create_resp.is_success());
        let created_tag = create_resp.to_body::<ApiTag>().await;

        // 取得
        let get_resp = client.get(&app, &created_tag.id).await;
        assert!(get_resp.is_success());
        let get_tag = get_resp.to_body::<ApiTag>().await;
        assert_eq!(get_tag.id, created_tag.id);

        // 更新
        let update_request = ApiTagUpdateBody {
            name: Some("更新後のタグ".to_string()),
            color: Some(255),
        };
        let update_resp = client.update(&app, &created_tag.id, update_request).await;
        assert!(update_resp.is_success());
        let updated_tag = update_resp.to_body::<ApiTag>().await;
        assert_eq!(updated_tag.name, "更新後のタグ");

        // 一覧取得
        let list_resp = client.list(&app).await;
        assert!(list_resp.is_success());
        let tags = list_resp.to_body::<Vec<ApiTag>>().await;
        assert!(tags.iter().any(|t| t.id == created_tag.id));

        // 削除
        let delete_resp = client.delete(&app, &created_tag.id).await;
        assert!(delete_resp.is_success());

        // 削除確認
        let get_deleted_resp = client.get(&app, &created_tag.id).await;
        assert!(!get_deleted_resp.is_success());
        assert_eq!(get_deleted_resp.status(), 404);
    }
}
