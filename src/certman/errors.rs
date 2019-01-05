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
    #[fail(display = "Invalid Path: {}", _0)]
    InvalidPath(String),

    #[fail(display = "Invalid File: {}", _0)]
    InvalidFile(String),

    #[fail(display = "Invalid Certificate: {}", _0)]
    InvalidCertificate(String),

    #[fail(display = "Unknown Error")]
    Unknown
}

impl From<actix::MailboxError> for Error {
    fn from(e: actix::MailboxError) -> Self {
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