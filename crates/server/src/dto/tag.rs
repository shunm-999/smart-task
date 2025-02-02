use crate::dto::{MapToApiModel, MapToDomain};
use domain::model::tag::{Tag, TagCreation, TagId, TagUpdating};
use openapi::model::tag::ApiTag;
use openapi::request::tag::{ApiTagCreateBody, ApiTagUpdateBody};

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

impl MapToDomain<TagCreation> for ApiTagCreateBody {
    fn map_to_domain(self) -> TagCreation {
        TagCreation {
            id: TagId::new(),
            name: self.name,
            color: self.color.map(|c| c.into()).unwrap_or_default(),
        }
    }
}

impl MapToDomain<TagUpdating> for (String, ApiTagUpdateBody) {
    fn map_to_domain(self) -> TagUpdating {
        let (id, body) = self;
        TagUpdating {
            id: (&id).into(),
            name: body.name,
            color: body.color.map(|c| c.into()),
        }
    }
}
