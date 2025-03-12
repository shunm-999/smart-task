use domain::model::task::Task;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "task", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_id: Option<String>,
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
    Todo,
    #[sea_orm(num_value = 1)]
    InProgress,
    #[sea_orm(num_value = 2)]
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
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id"
    )]
    Project,
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        super::tag_task::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::tag_task::Relation::Task.def().rev())
    }
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub struct TagTaskRelation(pub Model, pub Vec<super::tag::Model>);

impl From<(Model, Vec<super::tag::Model>)> for TagTaskRelation {
    fn from(value: (Model, Vec<crate::database::entity::tag::Model>)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<Status> for domain::model::task::TaskStatus {
    fn from(value: Status) -> Self {
        match value {
            Status::Todo => Self::Todo,
            Status::InProgress => Self::InProgress,
            Status::Done => Self::Done,
        }
    }
}

impl From<Priority> for domain::model::task::TaskPriority {
    fn from(value: Priority) -> Self {
        match value {
            Priority::Low => Self::Low,
            Priority::Medium => Self::Medium,
            Priority::High => Self::High,
        }
    }
}

impl From<TagTaskRelation> for Task {
    fn from(value: TagTaskRelation) -> Self {
        let task = value.0;
        let tags = value.1;
        let tags = tags.into_iter().map(|tag| tag.into()).collect();

        Self {
            id: (&task.id).into(),
            project_id: task.project_id.map(|id| (&id).into()),
            title: task.title,
            description: task.description,
            status: task.status.into(),
            priority: task.priority.into(),
            tags,
            deadline: task.deadline.map(|d| d.and_utc()),
            created_at: task.created_at.and_utc(),
            updated_at: task.updated_at.and_utc(),
        }
    }
}
