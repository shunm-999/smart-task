use std::fmt::Display;
use uuid::Uuid;

pub mod model;
pub mod repository;

pub enum Error {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
    TooManyRequests,
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
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Into<String> for $name {
            fn into(self) -> String {
                self.0.to_string()
            }
        }

        impl From<&String> for $name {
            fn from(value: &String) -> Self {
                Self(uuid::Uuid::parse_str(value).unwrap())
            }
        }

        impl From<&uuid::Uuid> for $name {
            fn from(value: &uuid::Uuid) -> Self {
                Self(*value)
            }
        }

        impl AsRef<uuid::Uuid> for $name {
            fn as_ref(&self) -> &uuid::Uuid {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
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
