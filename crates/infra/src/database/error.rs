use domain::Error;
use sea_orm::{DbErr, TransactionError};

pub(crate) fn map_db_error_to_domain_error(error: DbErr) -> Error {
    match error {
        DbErr::ConnectionAcquire(err) => Error::InternalServerError,
        DbErr::TryIntoErr { from, into, source } => Error::InternalServerError,
        DbErr::Conn(err) => Error::InternalServerError,
        DbErr::Exec(err) => Error::InternalServerError,
        DbErr::Query(err) => Error::InternalServerError,
        DbErr::ConvertFromU64(err) => Error::InternalServerError,
        DbErr::UnpackInsertId => Error::InternalServerError,
        DbErr::UpdateGetPrimaryKey => Error::InternalServerError,
        DbErr::RecordNotFound(err) => Error::NotFound,
        DbErr::AttrNotSet(err) => Error::InternalServerError,
        DbErr::Custom(err) => Error::InternalServerError,
        DbErr::Type(err) => Error::InternalServerError,
        DbErr::Json(err) => Error::InternalServerError,
        DbErr::Migration(err) => Error::InternalServerError,
        DbErr::RecordNotInserted => Error::InternalServerError,
        DbErr::RecordNotUpdated => Error::InternalServerError,
    }
}

pub(crate) fn map_db_transaction_error_to_domain_error(error: TransactionError<DbErr>) -> Error {
    match error {
        TransactionError::Connection(err) => map_db_error_to_domain_error(err),
        TransactionError::Transaction(err) => map_db_error_to_domain_error(err),
    }
}
