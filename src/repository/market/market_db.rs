use crate::repository::{
    error::PersistanceError
};
use super::Market;
use r2d2_sqlite::rusqlite::{
    Row,
    params,
    NO_PARAMS
};
use crate::repository::DbConn;

impl From<&Row<'_>> for Market {
    fn from(row: &Row) -> Self {
        Market {
            id: row.get_unwrap(0),
            symbol: row.get_unwrap(1),
        }
    }
}

pub fn create_table_if_not_exists(db: &DbConn) -> Result<(), PersistanceError> {
    db.execute(
        r"CREATE TABLE IF NOT EXISTS market (
            id INTEGER PRIMARY KEY,
            symbol VARCHAR(4)
        )", NO_PARAMS)
        .map(|x| ())
        .map_err(|e| PersistanceError::InitializationError(e))
}

pub fn add(db: &DbConn, market: &Market) -> Result<(), PersistanceError> {
    let result = db.execute(
        r"INSERT INTO 
            market (symbol) 
            values (?1);",
        params![market.symbol]);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PersistanceError::CouldNotInsert(e))
    }
}

pub fn delete(db: &DbConn, id: u16) -> Result<(), PersistanceError> {
    let result = db.execute(
        r"DELETE FROM market 
            WHERE id = ?1;", 
        params![id]);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PersistanceError::CouldNotDelete(e))
    }
}

pub fn update(db: &DbConn, market: &Market) -> Result<(), PersistanceError> {
    let result = db.execute(r"
        UPDATE market 
            SET symbol = ?1
            WHERE id = ?2",
        params![market.symbol, market.id.to_string() ]);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PersistanceError::CouldNotUpdate(e))
    }
            
}

pub fn get(db: &DbConn, id: u16) -> Result<Option<Market>, PersistanceError> {
    let result = db.query_row(
        "SELECT id, symbol FROM market WHERE id = ?1", 
        params![id], 
        |row| {
            Ok(Some(Market {
                id: row.get(0).unwrap(),
                symbol: row.get(1).unwrap(),
            }))
        });

    result.map_err(|error| PersistanceError::CouldNotInsert(error))
}

pub fn get_all(db: &DbConn) -> std::result::Result<Vec<Market>, PersistanceError> { 
    let mut query = db.prepare(r"
    SELECT id, symbol
        FROM market").unwrap();

    let items = query.query_map(
        NO_PARAMS, 
        |row| Ok(Market::from(row)))
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    Ok(items)
}
