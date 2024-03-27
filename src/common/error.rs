use sea_orm::DbErr;

#[derive(Debug)]
pub enum MyError {
    DatabaseError(DbErr),
    ValidationError(String),
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
        }
    }
}

impl std::error::Error for MyError {}