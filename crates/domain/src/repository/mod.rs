use crate::repository::project::ProjectRepository;
use crate::repository::tag::TagRepository;
use crate::repository::task::TaskRepository;

pub mod project;
pub mod tag;
pub mod task;

pub trait SmartTaskRepository: TagRepository + TaskRepository + ProjectRepository {}
