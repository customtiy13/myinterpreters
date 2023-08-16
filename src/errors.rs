#[derive(Debug)]
pub enum MyError {
    ParseError(String),
    CastError(String),
    DividedbyzeroError,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MyError::ParseError(ref err) => write!(f, "Parsing error occurred {:?}", err),
            MyError::CastError(ref err) => write!(f, "Casting error occurred {:?}", err),
            MyError::DividedbyzeroError => write!(f, "Divided by zero Error occurred"),
        }
    }
}

impl std::error::Error for MyError {}
