#[derive(Fail, Debug)]
pub enum Error {

    #[fail(display = "Not Authenticated")]
    NotAuthenticated,

    #[fail(display = "Missing Required Permission {} for {}", _0, _1)]
    NotAuthorized(String, String),

    #[fail(display = "Missing Resource Parameter: {}", _0)]
    MissingResourceParameter(String),
/*
    #[fail(display = "Unknown Error")]
    Unknown
*/
}