use domain::repository::SmartTaskRepository;
use infra::SmartTaskRepositoryImpl;
use std::sync::Arc;

mod error;

pub struct SmartTaskServer {
    pub repository: Arc<SmartTaskRepositoryImpl>,
}
