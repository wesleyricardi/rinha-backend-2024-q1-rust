pub enum Error {
    NotFound,
    TransactionDenied,
    Other,
}

impl Error {
    pub fn not_found<T>() -> Result<T, Error> {
        Err(Error::NotFound)
    }
    pub fn transaction_denied<T>() -> Result<T, Error> {
        Err(Error::TransactionDenied)
    }
    pub fn other<T>() -> Result<T, Error> {
        Err(Error::Other)
    }
}

