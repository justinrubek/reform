use failure::Fail;

#[derive(Clone, Debug, Fail)]
pub enum Error {
    /// 401
    #[fail(display = "Unauthorized")]
    Unauthorized,

    /// 403
    #[fail(display = "Forbidden")]
    Forbidden,

    /// 404
    #[fail(display = "Not Found")]
    NotFound,

    /// 500
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    /// serde deserialize error
    #[fail(display = "Deserialize Error")]
    DeserializeError,

    /// request error
    #[fail(display = "HTTP Request Error")]
    RequestError,
}
