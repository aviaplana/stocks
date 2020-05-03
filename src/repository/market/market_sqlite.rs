use crate::repository::{
    Crud,
    error::PersistanceError
};
use super::Market;
use std::rc::Rc;
use rusqlite::{
    Connection, 
    params,
    NO_PARAMS
};

impl From<&rusqlite::Row<'_>> for Market {
    fn from(row: &rusqlite::Row) -> Self {
        Market {
            id: row.get_unwrap(0),
            symbol: row.get_unwrap(1),
        }
    }
}

pub struct MarketSqlite {
    db: Rc<Connection>
}

impl MarketSqlite {
    pub fn new(db: Rc<Connection>) -> MarketSqlite {
        db.execute(
            r"CREATE TABLE IF NOT EXISTS market (
                id INTEGER PRIMARY KEY,
                symbol VARCHAR(4)
            )", NO_PARAMS)
            .map_err(|e| PersistanceError::InitializationError(e))
            .unwrap();
        
        MarketSqlite { db }
    }
}

impl Crud for MarketSqlite {
    type Item = Market;
    type IdType = u16;

    fn add(&self, market: &Self::Item) -> Result<(), PersistanceError> {
        let result = self.db.execute(
            r"INSERT INTO 
                market (symbol) 
                values (?1);",
            params![market.symbol]);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PersistanceError::CouldNotInsert(e))
        }
    }

    fn delete(&self, id: Self::IdType) -> Result<(), PersistanceError> {
        let result = self.db.execute(
            r"DELETE FROM market 
                WHERE id = ?1;", 
            params![id]);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PersistanceError::CouldNotDelete(e))
        }
    }

    fn update(&self, market: &Self::Item) -> Result<(), PersistanceError> {
        let result = self.db.execute(r"
            UPDATE market 
                SET symbol = ?1
                WHERE id = ?2",
            params![market.symbol, market.id.to_string() ]);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PersistanceError::CouldNotUpdate(e))
        }
                
    }

    fn get(&self, id: Self::IdType) -> Result<Option<Self::Item>, PersistanceError> {
        let result = self.db.query_row(
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
    
    fn get_all(&self) -> std::result::Result<Vec<Self::Item>, PersistanceError> { 
        let mut query = self.db.prepare(r"
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
}