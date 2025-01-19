use crate::model::tag::{Tag, TagCreation, TagId, TagUpdating};
use crate::Result;

pub trait TagRepository {
    async fn get_tags(&self) -> Result<Vec<Tag>>;
    async fn get_tag(&self, tag_id: TagId) -> Result<Option<Tag>>;
    async fn create_tag(&self, tag_creation: TagCreation) -> Result<Tag>;
    async fn update_tag(&self, tag_updating: TagUpdating) -> Result<Tag>;
    async fn delete_tag(&self, tag_id: TagId) -> Result<Tag>;
}
