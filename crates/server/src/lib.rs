use domain::repository::SmartTaskRepository;
use infra::SmartTaskRepositoryImpl;
use std::sync::Arc;

pub mod endpoint;
mod error;

#[derive(Clone)]
pub struct SmartTaskServer {
    pub repository: Arc<SmartTaskRepositoryImpl>,
}
