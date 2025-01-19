use crate::repository::SmartTaskRepository;
use domain::model::tag::{Tag, TagCreation, TagId, TagUpdating};
use domain::repository::tag::TagRepository;

impl TagRepository for SmartTaskRepository {
    async fn get_tags(&self) -> domain::Result<Vec<Tag>> {
        todo!()
    }

    async fn get_tag(&self, tag_id: TagId) -> domain::Result<Option<Tag>> {
        todo!()
    }

    async fn create_tag(&self, tag_creation: TagCreation) -> domain::Result<Tag> {
        todo!()
    }

    async fn update_tag(&self, tag_updating: TagUpdating) -> domain::Result<Tag> {
        todo!()
    }

    async fn delete_tag(&self, tag_id: TagId) -> domain::Result<Tag> {
        todo!()
    }
}
