#[derive(Fail, Debug)]
pub enum Error {
    /*
    #[fail(display = "Data Conflict")]
    DataConflict(String),

    #[fail(display = "Data Not Found")]
    DataNotFound(String),

    #[fail(display = "Incorrect Query")]
    DataIncorrect(String),

    #[fail(display = "Unknown Error")]
    Unknown(diesel::result::Error)
    */
    #[fail(display = "File Error: {}", _0)]
    FileError(std::io::Error),

    #[fail(display = "Parse Error")]
    ParseError,

    #[fail(display = "Invalid Certificate: {}", _0)]
    InvalidCertificate(String),

    #[fail(display = "Database Error: {}", _0)]
    DatabaseError(crate::errors::ServiceError),

    #[fail(display = "Unknown Error")]
    Unknown
}

impl From<actix::MailboxError> for Error {
    fn from(_: actix::MailboxError) -> Self {
        Error::Unknown
    }
}

// This really should be a DatabaseError implementaiton thing..?
impl From<crate::errors::ServiceError> for Error {
    fn from(_: crate::errors::ServiceError) -> Self {
        Error::Unknown
    }
}

/*
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
*/