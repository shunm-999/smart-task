use crate::database::error::map_db_error_to_domain_error;
use migration::MigratorTrait;
use sea_orm::{Database, DatabaseConnection, DatabaseTransaction, TransactionTrait};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub(crate) struct DatabaseConnectionProvider {
    conn: DatabaseConnection,
}

impl<'a> DatabaseConnectionProvider {
    pub(crate) fn get_connection(&'a self) -> &'a DatabaseConnection {
        &self.conn
    }

    pub(crate) async fn transaction<F, T>(&'a self, callback: F) -> domain::Result<T>
    where
        for<'c> F: FnOnce(
            &'c DatabaseTransaction,
        ) -> Pin<Box<dyn Future<Output = domain::Result<T>> + Send + 'c>>,
    {
        let conn = self.get_connection();
        let txn = conn
            .begin()
            .await
            .map_err(|e| map_db_error_to_domain_error(e))?;

        let result = callback(&txn).await?;

        txn.commit()
            .await
            .map_err(|e| map_db_error_to_domain_error(e))?;
        Ok(result)
    }
}

impl DatabaseConnectionProvider {
    pub(crate) async fn new(db_url: String) -> Self {
        let conn = Database::connect(&db_url).await.unwrap();
        migration::Migrator::up(&conn, None).await.unwrap();
        Self { conn }
    }
}
