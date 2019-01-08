#[derive(Fail, Debug)]
pub enum Error {
/*
    #[fail(display = "Not Authenticated")]
    NotAuthenticated,

    #[fail(display = "Missing Required Permission {} for {}", _0, _1)]
    NotAuthorized(String, String),

    #[fail(display = "Missing Resource Parameter: {}", _0)]
    MissingResourceParameter(String),
    #[fail(display = "Unknown Error")]
    Unknown
*/
    #[fail(display = "IO Error: {}", _0)]
    IoError(std::io::Error),

    #[fail(display = "Name Error")]
    NameError
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}