use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "task", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
    pub deadline: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Status {
    #[sea_orm(num_value = 0)]
    Inbox,
    #[sea_orm(num_value = 1)]
    NextAction,
    #[sea_orm(num_value = 2)]
    Waiting,
    #[sea_orm(num_value = 3)]
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Priority {
    #[sea_orm(num_value = 0)]
    Low,
    #[sea_orm(num_value = 1)]
    Medium,
    #[sea_orm(num_value = 2)]
    High,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
