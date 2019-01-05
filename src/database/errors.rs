use diesel::result::{
    Error::{NotFound, DatabaseError, QueryBuilderError, SerializationError, DeserializationError},
};

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Data Conflict")]
    DataConflict(String),

    #[fail(display = "Data Not Found")]
    DataNotFound(String),

    #[fail(display = "Incorrect Query")]
    DataIncorrect(String),

    #[fail(display = "Unknown Error")]
    Unknown(diesel::result::Error)
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
            _ => Error::Unknown(e)
        }
    }
}