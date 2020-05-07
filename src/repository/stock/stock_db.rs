use crate::repository::{
    error::PersistanceError
};
use super::Stock;

use r2d2_sqlite::rusqlite::{ 
    params,
    NO_PARAMS,
    Row,
};
use crate::repository::DbConn;

impl From<&Row<'_>> for Stock {
    fn from(row: &Row) -> Self {
        Stock {
            symbol: row.get_unwrap(0),
            name: row.get_unwrap(1),
            price: row.get_unwrap::<_, f64>(2) as f32,
            initial_price: row.get_unwrap::<_, f64>(3) as f32,
            market: row.get_unwrap(4),
        }
    }
}

pub fn create_table_if_not_exists(db: &DbConn) -> Result<(), PersistanceError> {
    db.execute(
        r"CREATE TABLE IF NOT EXISTS stock (
            symbol VARCHAR(4) PRIMARY KEY,
            name VARCHAR(255),
            price REAL,
            initial_price REAL,
            market_id INTEGER REFERENCES market(id)
        )", NO_PARAMS)
    .map(|x| ())
    .map_err(|e| PersistanceError::InitializationError(e))
}

pub fn get_by_makret(db: &DbConn, market_id: u16) -> Result<Vec<Stock>, PersistanceError> {
    let mut query = db.prepare(r"
        SELECT symbol, name, price, initial_price, market_id
            FROM stock
            WHERE s.market_id = ?1").unwrap();
    
    let items = query.query_map(
        params![market_id], 
        |row| Ok(Stock::from(row)))
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    Ok(items)
}

pub fn update_price(db: &DbConn, symbol: &str, price: f32) -> Result<(), PersistanceError> {
    let result = db.execute(r"
        UPDATE stock
            SET price = ?1
            WHERE symbol = ?2",
        params![symbol, price.to_string()]);
    
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PersistanceError::CouldNotUpdate(e))
    }
}

pub fn add(db: &DbConn, stock: &Stock) -> Result<(), PersistanceError> {
    let result = db.execute(
        r"INSERT INTO 
            stock (symbol, name, price, initial_price, market_id) 
            values (?1, ?2, ?3, ?4, ?5);",
        params![
            stock.symbol, 
            stock.name, 
            stock.price.to_string(), 
            stock.initial_price.to_string(), 
            &stock.market.to_string()]);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PersistanceError::CouldNotInsert(e))
    }
}

pub fn delete(db: &DbConn, id: &str) -> Result<(), PersistanceError> {
    let result = db.execute(
        r"DELETE FROM stock 
            WHERE symbol = ?1;", 
        params![id]);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PersistanceError::CouldNotDelete(e))
    }
}

pub fn update(db: &DbConn, stock: &Stock) -> Result<(), PersistanceError> {
    let result = db.execute(r"
        UPDATE stock 
            SET name = ?1, price = ?2, initial_price = ?3, market_id = ?4
            WHERE symbol = ?5",
        params![
            stock.name,
            stock.price.to_string(), 
            stock.initial_price.to_string(), 
            stock.market.to_string(), 
            stock.symbol]);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PersistanceError::CouldNotUpdate(e))
    }
            
}

fn get(db: &DbConn, id: &str) -> Result<Option<Stock>, PersistanceError> {
    let result = db.query_row(
        "SELECT symbol, name, price, initial_price, market_id FROM stock WHERE symbol = ?1", 
        params![id], 
        |row| Ok(Some(Stock::from(row))));
    // TODO: Check if none is returned if id not found
    result.map_err(|error| PersistanceError::CouldNotInsert(error))
}

pub fn get_all(db: &DbConn) -> Result<Vec<Stock>, PersistanceError> {
    let mut query = db.prepare(r"
    SELECT symbol, name, price, initial_price, market_id
        FROM stock").unwrap();

    let items = query.query_map(
        NO_PARAMS, 
        |row| Ok(Stock::from(row)))
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    Ok(items)
}
