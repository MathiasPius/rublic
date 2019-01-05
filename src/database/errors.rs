use diesel::result::{
    Error::{NotFound, DatabaseError, QueryBuilderError, SerializationError, DeserializationError},
};

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Data Conflict: {}", _0)]
    DataConflict(String),

    #[fail(display = "Data Not Found: {}", _0)]
    DataNotFound(String),

    #[fail(display = "Incorrect Query: {}", _0)]
    DataIncorrect(String),

    #[fail(display = "Diesel Error: {}", _0)]
    DieselError(diesel::result::Error),

    #[fail(display = "Unknown Error")]
    Unknown
}

impl From<actix::MailboxError> for Error {
    fn from(_: actix::MailboxError) -> Self {
        Error::Unknown
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            DatabaseError(_kind, info) => {
                Error::DataConflict((*info).message().into())
            },
            NotFound => Error::DataNotFound("the query returned no results".into()),
            QueryBuilderError(err) => Error::DataIncorrect((*err).to_string()),
            SerializationError(err) => Error::DataIncorrect((*err).to_string()),
            DeserializationError(err) => Error::DataIncorrect((*err).to_string()),
            _ => Error::DieselError(e)
        }
    }
}