use sea_orm::{Database, DatabaseConnection};

pub(crate) struct DatabaseConnectionProvider {
    conn: DatabaseConnection,
}

impl<'a> DatabaseConnectionProvider {
    pub(crate) fn get_connection(&'a self) -> &'a DatabaseConnection {
        &self.conn
    }
}

impl DatabaseConnectionProvider {
    pub(crate) async fn new(db_url: String) -> Self {
        let conn = Database::connect(&db_url).await.unwrap();
        Self { conn }
    }
}
