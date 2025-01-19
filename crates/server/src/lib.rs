mod error;

use openapi::model::task::{ApiTask, ApiTaskPriority, ApiTaskStatus};

pub fn test() {
    let task = ApiTask::new(
        "1".to_string(),
        "title".to_string(),
        "description".to_string(),
        ApiTaskStatus::Done,
        ApiTaskPriority::High,
        vec![],
        chrono::Utc::now(),
        chrono::Utc::now(),
    );
}
