#[derive(Debug)]
pub enum MyError {
    ParseError(String),
    CastError(String),
    DividedbyzeroError,
    NotImplementedError,
    EnValueNotFoundError(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MyError::ParseError(ref err) => write!(f, "Parsing error occurred {:?}", err),
            MyError::CastError(ref err) => write!(f, "Casting error occurred {:?}", err),
            MyError::DividedbyzeroError => write!(f, "Divided by zero Error occurred"),
            MyError::NotImplementedError => write!(f, "Not implemented Error occurred"),
            MyError::EnValueNotFoundError(ref err) => write!(f, "Undefined variable {}.", err),
        }
    }
}

impl std::error::Error for MyError {}
