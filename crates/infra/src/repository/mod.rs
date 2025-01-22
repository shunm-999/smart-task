use crate::database::connection_provider::DatabaseConnectionProvider;
use domain::repository::SmartTaskRepository;
use sea_orm::ActiveValue;

mod project;
mod tag;
mod task;

pub struct DatabaseRepository {
    database_connection_provider: DatabaseConnectionProvider,
}

impl SmartTaskRepository for DatabaseRepository {}
