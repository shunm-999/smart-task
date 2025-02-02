use crate::dto::MapToApiModel;
use domain::model::tag::Tag;
use openapi::model::tag::ApiTag;

impl MapToApiModel<ApiTag> for Tag {
    fn map_to_api_model(self) -> ApiTag {
        ApiTag {
            id: self.id.to_string(),
            name: self.name.clone(),
            color: self.color.into(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
