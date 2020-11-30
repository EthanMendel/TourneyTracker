#[derive(Debug)]
pub enum TourneyError {
    DbError(String),
    InternalServerError(String),
    Unauthorized,
}

impl std::fmt::Display for TourneyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TourneyError::DbError(err) => write!(f, "Database error: {}", err),
            TourneyError::InternalServerError(err) => write!(f, "Internal server error: {}", err),
            TourneyError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl<E: std::error::Error> From<E> for TourneyError {
    fn from(e: E) -> Self {
        TourneyError::InternalServerError(format!("{:?}", e))
    }
}
