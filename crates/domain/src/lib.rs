pub mod model;
pub mod repository;

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

pub type Result<T> = std::result::Result<T, Error>;

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

#[macro_export]
macro_rules! data_enum {
    ($name:ident, $($variant:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub enum $name {
            $(
                $variant,
            )*
        }
    };
}

#[macro_export]
macro_rules! data_model {
    ($name:ident, $($field:ident: $type:ty),*) => {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            $(
                pub $field: $type,
            )*
        }

        impl $name {
            pub fn new($($field: $type),*) -> Self {
                Self {
                    $(
                        $field,
                    )*
                }
            }
        }
    };
}
