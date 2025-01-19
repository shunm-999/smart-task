mod model;

pub enum Error {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
    TooManyRequests,
    NotDeletableResource,
    ServiseMaintenance,
}

#[macro_export]
macro_rules! data_id {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
            serde::Serialize, serde::Deserialize
        )]
        pub struct $name(uuid::Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(uuid::Uuid::now_v7())
            }

            pub fn from_string(value: &str) -> Self {
                Self(uuid::Uuid::parse_str(value).unwrap())
            }
        }

        impl Into<String> for $name {
            fn into(self) -> String {
                self.0.to_string()
            }
        }
    };
}
