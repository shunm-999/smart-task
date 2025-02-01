use crate::database::connection_provider::DatabaseConnectionProvider;
use domain::repository::SmartTaskRepository;

mod database;
mod repository;

#[derive(Clone)]
pub struct SmartTaskRepositoryImpl {
    database_connection_provider: DatabaseConnectionProvider,
}

impl SmartTaskRepository for SmartTaskRepositoryImpl {}

impl SmartTaskRepositoryImpl {
    pub async fn new(db_url: String) -> Self {
        let database_connection_provider = DatabaseConnectionProvider::new(db_url).await;
        Self {
            database_connection_provider,
        }
    }
}
