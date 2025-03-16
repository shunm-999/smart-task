#[cfg(test)]
mod tests {
    use crate::client::tag::TagClient;
    use crate::endpoint::test_helper::setup_test_app;
    use actix_web::test;
    use openapi::model::tag::ApiTag;

    #[actix_web::test]
    async fn test_tag_create() {
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
}
