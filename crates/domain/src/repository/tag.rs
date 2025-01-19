use crate::model::tag::{Tag, TagCreation, TagId, TagUpdating};
use crate::Result;

pub trait TagRepository {
    fn get_tags(&self) -> Result<Vec<Tag>>;
    fn get_tag(&self, tag_id: TagId) -> Result<Option<Tag>>;
    fn create_tag(&self, tag_creation: TagCreation) -> Result<Tag>;
    fn update_tag(&self, tag_updating: TagUpdating) -> Result<Tag>;
    fn delete_tag(&self, tag_id: TagId) -> Result<Tag>;
}
