#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "File Error: {}", _0)]
    FileError(std::io::Error),

    #[fail(display = "Certificate Parse Error")]
    ParseError,

    #[fail(display = "Invalid Certificate: {}", _0)]
    InvalidCertificate(String),

    #[fail(display = "Service Error: {}", _0)]
    ServiceError(crate::errors::ServiceError),

    #[fail(display = "Database Error: {}", _0)]
    DatabaseError(crate::database::errors::Error),

    #[fail(display = "Unknown Error")]
    Unknown
}

impl From<actix::MailboxError> for Error {
    fn from(_: actix::MailboxError) -> Self {
        Error::Unknown
    }
}

impl From<crate::database::errors::Error> for Error {
    fn from(e: crate::database::errors::Error) -> Self {
        Error::DatabaseError(e)
    }
}

// This really should be a DatabaseError implementaiton thing..?
impl From<crate::errors::ServiceError> for Error {
    fn from(_: crate::errors::ServiceError) -> Self {
        Error::Unknown
    }
}