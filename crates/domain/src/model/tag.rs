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

impl From<TagColor> for i32 {
    fn from(color: TagColor) -> Self {
        i32::from(color.r) << 16 | i32::from(color.g) << 8 | i32::from(color.b)
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
    pub id: TagId,
    pub name: String,
    pub color: TagColor,
}

pub struct TagUpdating {
    pub id: TagId,
    pub name: Option<String>,
    pub color: Option<TagColor>,
}
