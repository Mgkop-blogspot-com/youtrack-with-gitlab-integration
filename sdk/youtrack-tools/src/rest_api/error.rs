use core::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HttpError(hyper::Error),
    RestError{
        status:u16,
        id: String
    },
    ConverterError(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::HttpError(e) => {e.fmt(f)}
            Error::RestError{status:404u16, id} => {
                write!(f, "Entry with id: {} not found", id)
            },
            Error::ConverterError(e) => e.fmt(f),
            e @ Error::RestError{..} => {
                write!(f, "Some rest error: {:?}", e)
            }
        }
    }
}

