use crate::database::entity::tag::{ActiveModel, Entity as TagEntity, Model};
use crate::database::error::map_to_domain_error;
use crate::repository::DatabaseRepository;
use domain::model::tag::{Tag, TagCreation, TagId, TagUpdating};
use domain::repository::tag::TagRepository;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait};

impl TagRepository for DatabaseRepository {
    async fn get_tags(&self) -> domain::Result<Vec<Tag>> {
        let conn = self.database_connection_provider.get_connection();
        let tags = TagEntity::find()
            .all(conn)
            .await
            .map_err(|e| map_to_domain_error(e))?;

        tags.into_iter().map(|tag| Ok(tag.into())).collect()
    }

    async fn get_tag(&self, tag_id: TagId) -> domain::Result<Tag> {
        let conn = self.database_connection_provider.get_connection();
        let tag = TagEntity::find_by_id(tag_id.to_string())
            .one(conn)
            .await
            .map_err(|e| map_to_domain_error(e))?
            .ok_or(domain::Error::NotFound)?;

        Ok(tag.into())
    }

    async fn create_tag(&self, tag_creation: TagCreation) -> domain::Result<Tag> {
        let conn = self.database_connection_provider.get_connection();
        let tag: ActiveModel = tag_creation.into();
        let tag = tag.insert(conn).await.map_err(|e| map_to_domain_error(e))?;

        Ok(tag.into())
    }

    async fn update_tag(&self, tag_updating: TagUpdating) -> domain::Result<Tag> {
        let conn = self.database_connection_provider.get_connection();
        let tag = TagEntity::find_by_id(tag_updating.id.to_string())
            .one(conn)
            .await
            .map_err(|e| map_to_domain_error(e))?
            .ok_or(domain::Error::NotFound)?;

        let tag: ActiveModel = (tag, tag_updating).into();
        let tag = tag.update(conn).await.map_err(|e| map_to_domain_error(e))?;

        Ok(tag.into())
    }

    async fn delete_tag(&self, tag_id: TagId) -> domain::Result<Tag> {
        let conn = self.database_connection_provider.get_connection();
        let tag = TagEntity::find_by_id(tag_id.to_string())
            .one(conn)
            .await
            .map_err(|e| map_to_domain_error(e))?
            .ok_or(domain::Error::NotFound)?;

        tag.clone()
            .delete(conn)
            .await
            .map_err(|e| map_to_domain_error(e))?;

        Ok(tag.into())
    }
}

impl From<TagCreation> for ActiveModel {
    fn from(tag_creation: TagCreation) -> Self {
        ActiveModel {
            id: Set(tag_creation.id.to_string()),
            name: Set(tag_creation.name),
            color_r: Set(tag_creation.color.r as i32),
            color_g: Set(tag_creation.color.g as i32),
            color_b: Set(tag_creation.color.b as i32),
            ..Default::default()
        }
    }
}

impl From<(Model, TagUpdating)> for ActiveModel {
    fn from((tag, tag_updating): (Model, TagUpdating)) -> Self {
        let mut tag: ActiveModel = tag.into();

        if let Some(name) = tag_updating.name {
            tag.name = Set(name);
        }
        if let Some(color) = tag_updating.color {
            tag.color_r = Set(color.r as i32);
            tag.color_g = Set(color.g as i32);
            tag.color_b = Set(color.b as i32);
        }
        tag.updated_at = Set(chrono::Utc::now().naive_utc());
        tag
    }
}
