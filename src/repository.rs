mod market;
mod stock;
mod error;

use std::rc::Rc;
use error::PersistanceError;
use stock::{
    Stock,
    stock_sqlite::StockSqlite
};
use market::{
    Market,
    market_sqlite::MarketSqlite
};
use rusqlite::Connection;

trait Crud {
    type Item;
    type IdType;

    fn add(&self, item: &Self::Item) -> Result<(), PersistanceError>;
    fn delete(&self, id: Self::IdType) -> Result<(), PersistanceError>;
    fn update(&self, id: &Self::Item) -> Result<(), PersistanceError>;
    fn get(&self, id: Self::IdType) -> Result<Option<Self::Item>, PersistanceError>;
    fn get_all(&self) -> Result<Vec<Self::Item>, PersistanceError>;
}

pub struct StockRepository {
    stock_ds: StockSqlite,
    market_ds: MarketSqlite,
}

impl StockRepository {
    pub fn new() -> Result<StockRepository, PersistanceError> {
        match Connection::open("stocks.db") {
            Ok(db_conn) => {
                let db = Rc::new(db_conn);
                
                let stock_ds = StockSqlite::new(db.clone());
                let market_ds = MarketSqlite::new(db.clone());

                Ok(
                    StockRepository {
                        stock_ds,
                        market_ds,
                    }
                )
            },
            Err(e) => Err(PersistanceError::InitializationError(e)) // TODO: Check if error is correct
        }
    }

    pub fn add_stock(&self, stock: &Stock) -> Result<(), PersistanceError> {
        self.stock_ds.add(stock)
    }

    pub fn add_market(&self, market: &Market) -> Result<(), PersistanceError> {
        self.market_ds.add(market)
    }

    pub fn update_stock(&self, stock: &Stock) -> Result<(), PersistanceError> {
        self.stock_ds.update(stock)
    }

    pub fn update_market(&self, market: &Market) -> Result<(), PersistanceError> {
        self.market_ds.update(market)
    }

    pub fn delete_stock(&self, symbol: &str) -> Result<(), PersistanceError> {
        self.stock_ds.delete(symbol.to_owned())
    }

    pub fn delete_makret(&self, id: u16) -> Result<(), PersistanceError> {
        if self.stock_ds.get_by_makret(id)?.is_empty() {
            self.market_ds.delete(id)
        } else {
            Err(PersistanceError::EntryHasDependencies)
        }
    }
/*
    pub fn get_entries(&self) -> Vec<Stock> {
        let mut stmt = self.db.prepare(
            "SELECT s.* FROM stock s;",).unwrap();
        
        stmt.query_map(NO_PARAMS, |row| {
            Ok(Stock {
                symbol: row.get(0).unwrap(),
                price: row.get::<_, f64>(1).unwrap() as f32,
                initial_price: row.get::<_, f64>(2).unwrap() as f32,
                market: row.get(3).unwrap(),
            })
        }).unwrap().map(|x| x.unwrap()).collect::<Vec<Stock>>()
    }*/
}


/*
#[test]
fn test_add() {
    let mut persistance = StockRepository::new().unwrap();
    
    let mock_asset = get_asset_mock();

    persistance.add(mock_asset.clone());
    let entries = persistance.get_entries();

    assert!(entries.iter().any(|x| x == &mock_asset));
}

#[test]
fn test_update() {
    let mut persistance = StockRepository::new().unwrap();
    
    let mut mock_asset = get_asset_mock();

    persistance.add(mock_asset.clone());

    let new_price = 12332.3;
    mock_asset.set_price(new_price);
    assert!(persistance.update(&mock_asset).is_ok());

    let entries = persistance.get_entries();

    assert!(entries.iter()
        .find(|&x| x.symbol == mock_asset.symbol)
        .unwrap()
        .price == new_price);
}


#[test]
fn test_delete() {
    let mut persistance = StockRepository::new().unwrap();
    
    let mock_asset = get_asset_mock();

    persistance.add(mock_asset.clone());

    assert!(persistance.delete(&mock_asset.symbol).is_ok());
}*/
/*

fn get_asset_mock() -> Stock {
    Stock {
        symbol: String::from("BTC"),
        price: 8000.0,
        initial_price: 7990.0,
        market: 1
    }
}
*/