use std::{
    error::Error,
    fmt,
};
use r2d2_sqlite::rusqlite;

#[derive(Debug)]
pub enum PersistanceError {
    KeyNotFoundError,
    InitializationError(rusqlite::Error),
    CouldNotInsert(rusqlite::Error),
    CouldNotDelete(rusqlite::Error),
    CouldNotUpdate(rusqlite::Error),
    EntryHasDependencies,
}

impl Error for PersistanceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PersistanceError::KeyNotFoundError => None,
            PersistanceError::EntryHasDependencies => None,
            PersistanceError::CouldNotInsert(e) |
            PersistanceError::CouldNotUpdate(e) |
            PersistanceError::CouldNotDelete(e) |
            PersistanceError::InitializationError(e) => e.source()
        }
    }
}

impl fmt::Display for PersistanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PersistanceError::KeyNotFoundError => write!(f, "Key not found!"),
            PersistanceError::EntryHasDependencies => write!(f, "Some items depend on this item!"),
            PersistanceError::CouldNotInsert(e) |
            PersistanceError::CouldNotUpdate(e) |
            PersistanceError::CouldNotDelete(e) |
            PersistanceError::InitializationError(e) => write!(f, "{}", e.to_string())
        }
    }
}
