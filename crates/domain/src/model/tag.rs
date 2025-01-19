use crate::{data_id, data_model};

data_id!(TagId);

data_model!(
    TagColor,
    r: u8,
    g: u8,
    b: u8
);

impl Default for TagColor {
    fn default() -> Self {
        TagColor { r: 0, g: 0, b: 0 }
    }
}

data_model!(
    Tag,
    id: TagId,
    name: String,
    color: TagColor,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>
);

pub struct TagCreation {
    pub name: String,
    pub color: TagColor,
}

pub struct TagUpdating {
    pub id: TagId,
    pub name: Option<String>,
    pub color: Option<TagColor>,
}
