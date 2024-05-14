use sea_orm::DbErr;

#[derive(Debug)]
pub enum MyError {
    DatabaseError(DbErr),
    ValidationError(String),
    InvalidTypeError(String),
    AuthError(String),
    BadRequestError(String),
    NotFound(String),
    ConversionError(String),
    // 其他错误类型...
}

impl From<DbErr> for MyError {
    fn from(err: DbErr) -> MyError {
        MyError::DatabaseError(err)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MyError::DatabaseError(ref err) => write!(f, "Database error: {}", err),
            MyError::ValidationError(ref msg) => write!(f, "Validation error: {}", msg),
            MyError::InvalidTypeError(ref msg) => write!(f, "Invalid type error: {}", msg),
            MyError::AuthError(ref msg) => write!(f, "Auth error : {}", msg),
            MyError::BadRequestError(ref err) => write!(f, "Bad request error: {}", err),
            MyError::NotFound(ref err) => write!(f, "NotFound  error: {}", err),
            MyError::ConversionError(ref err) => write!(f, "ConversionError  error: {}", err),
        }
    }
}

impl std::error::Error for MyError {}
