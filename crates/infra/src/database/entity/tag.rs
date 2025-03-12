use domain::model::tag::{Tag, TagColor};
use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tag", rename_all = "snake_case")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub color_r: i32,
    pub color_g: i32,
    pub color_b: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::task::Entity> for Entity {
    fn to() -> RelationDef {
        super::tag_task::Relation::Task.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::tag_task::Relation::Tag.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Tag {
    fn from(value: Model) -> Self {
        let color = TagColor {
            r: value.color_r as u8,
            g: value.color_g as u8,
            b: value.color_b as u8,
        };
        Self {
            id: (&value.id).into(),
            name: value.name,
            color,
            created_at: value.created_at.and_utc(),
            updated_at: value.updated_at.and_utc(),
        }
    }
}
