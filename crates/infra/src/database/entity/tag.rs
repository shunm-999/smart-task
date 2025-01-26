use domain::model::tag::{Tag, TagColor};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tag", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub color_r: i32,
    pub color_g: i32,
    pub color_b: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Into<Tag> for Model {
    fn into(self) -> Tag {
        let color = TagColor {
            r: self.color_r as u8,
            g: self.color_g as u8,
            b: self.color_b as u8,
        };
        Tag {
            id: (&self.id).into(),
            name: self.name,
            color,
            created_at: self.created_at.and_utc(),
            updated_at: self.updated_at.and_utc(),
        }
    }
}
