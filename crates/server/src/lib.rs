use domain::repository::SmartTaskRepository;
use infra::SmartTaskRepositoryImpl;
use std::sync::Arc;

mod dto;
pub mod endpoint;

#[derive(Clone)]
pub struct AppData {
    pub repository: Arc<SmartTaskRepositoryImpl>,
}
