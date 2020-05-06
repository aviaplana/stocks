use crate::repository::{
    Crud,
    error::PersistanceError
};
use super::Stock;

use std::rc::Rc;
use rusqlite::{
    Connection, 
    params,
    NO_PARAMS
};

impl From<&rusqlite::Row<'_>> for Stock {
    fn from(row: &rusqlite::Row) -> Self {
        Stock {
            symbol: row.get_unwrap(0),
            name: row.get_unwrap(1),
            price: row.get_unwrap::<_, f64>(2) as f32,
            initial_price: row.get_unwrap::<_, f64>(3) as f32,
            market: row.get_unwrap(4),
        }
    }
}

pub struct StockSqlite {
    db: Rc<Connection>
}

impl StockSqlite {
    pub fn new(db: Rc<Connection>) -> StockSqlite {
        db.execute(
            r"CREATE TABLE IF NOT EXISTS stock (
                symbol VARCHAR(4) PRIMARY KEY,
                name VARCHAR(255),
                price REAL,
                initial_price REAL,
                market_id INTEGER REFERENCES market(id)
            )", NO_PARAMS)
        .map_err(|e| PersistanceError::InitializationError(e))
        .unwrap();
    
        StockSqlite { db }
    }

    pub fn get_by_makret(&self, market_id: u16) -> Result<Vec<Stock>, PersistanceError> {
        let mut query = self.db.prepare(r"
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

    pub fn update_price(&self, symbol: &str, price: f32) -> Result<(), PersistanceError> {
        let result = self.db.execute(r"
            UPDATE stock
                SET price = ?1
                WHERE symbol = ?2",
            params![symbol, price.to_string()]);
        
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PersistanceError::CouldNotUpdate(e))
        }
    }
}

impl Crud for StockSqlite {
    type Item = Stock;
    type IdType = String;

    fn add(&self, stock: &Self::Item) -> Result<(), PersistanceError> {
        let result = self.db.execute(
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

    fn delete(&self, id: Self::IdType) -> Result<(), PersistanceError> {
        let result = self.db.execute(
            r"DELETE FROM stock 
                WHERE symbol = ?1;", 
            params![id]);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PersistanceError::CouldNotDelete(e))
        }
    }

    fn update(&self, stock: &Self::Item) -> Result<(), PersistanceError> {
        let result = self.db.execute(r"
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

    fn get(&self, id: Self::IdType) -> Result<Option<Self::Item>, PersistanceError> {
        let result = self.db.query_row(
            "SELECT symbol, name, price, initial_price, market_id FROM stock WHERE symbol = ?1", 
            params![id], 
            |row| Ok(Some(Stock::from(row))));
        // TODO: Check if none is returned if id not found
        result.map_err(|error| PersistanceError::CouldNotInsert(error))
    }

    fn get_all(&self) -> Result<Vec<Self::Item>, PersistanceError> {
        let mut query = self.db.prepare(r"
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
}
