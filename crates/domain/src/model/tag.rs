use crate::{data_id, data_model};

data_id!(TagId);

data_model!(
    TagColor,
    r: u8,
    g: u8,
    b: u8
);

data_model!(
    Tag,
    id: TagId,
    name: String,
    color: TagColor,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>
);
