use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Project table
        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(pk_uuid(Project::Id))
                    .col(string(Project::Name))
                    .col(string(Project::Description))
                    .col(timestamp_with_time_zone(Project::CreatedAt))
                    .col(timestamp_with_time_zone(Project::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        // Tag table
        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(pk_uuid(Tag::Id))
                    .col(string(Tag::Name))
                    .col(integer(Tag::ColorR))
                    .col(integer(Tag::ColorG))
                    .col(integer(Tag::ColorB))
                    .col(timestamp_with_time_zone(Tag::CreatedAt))
                    .col(timestamp_with_time_zone(Tag::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        // Task table
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(pk_uuid(Task::Id))
                    .col(uuid_null(Task::ProjectId))
                    .col(string(Task::Title))
                    .col(string(Task::Description))
                    .col(integer(Task::Status))
                    .col(integer(Task::Priority))
                    .col(timestamp_with_time_zone_null(Task::Deadline))
                    .col(timestamp_with_time_zone(Task::CreatedAt))
                    .col(timestamp_with_time_zone(Task::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-project_id")
                            .from(Task::Table, Task::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // TagTask table
        manager
            .create_table(
                Table::create()
                    .table(TagTask::Table)
                    .if_not_exists()
                    .col(uuid(TagTask::TagId))
                    .col(uuid(TagTask::TaskId))
                    .primary_key(Index::create().col(TagTask::TagId).col(TagTask::TaskId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-tag_task-tag_id")
                            .from(TagTask::Table, TagTask::TagId)
                            .to(Tag::Table, Tag::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-tag_task-task_id")
                            .from(TagTask::Table, TagTask::TaskId)
                            .to(Task::Table, Task::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TagTask::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Name,
    ColorR,
    ColorG,
    ColorB,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    ProjectId,
    Title,
    Description,
    Status,
    Priority,
    Deadline,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum TagTask {
    Table,
    TagId,
    TaskId,
}
