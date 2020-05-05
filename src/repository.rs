mod market;
mod stock;
mod error;

use std::rc::Rc;
use error::PersistanceError;
use stock::{
    Stock,
    stock_api::{
        StockApi
    },
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
    stock_db: StockSqlite,
    stock_api: StockApi,
    market_db: MarketSqlite,
}

impl StockRepository {
    pub fn new() -> Result<StockRepository, PersistanceError> {
        let stock_api = StockApi::new();
        match Connection::open("stocks.db") {
            Ok(db_conn) => {
                let db = Rc::new(db_conn);
                
                let stock_db = StockSqlite::new(db.clone());
                let market_db = MarketSqlite::new(db.clone());

                Ok(
                    StockRepository {
                        stock_db,
                        stock_api,
                        market_db,
                    }
                )
            },
            Err(e) => Err(PersistanceError::InitializationError(e)) // TODO: Check if error is correct
        }
    }

    // Stores a stock in the local storage
    pub fn store_stock(&self, stock: &Stock) -> Result<(), PersistanceError> {
        self.stock_db.add(stock)
    }

    // Deletes a stock from the local storage
    pub fn delete_stock(&self, symbol: &str) -> Result<(), PersistanceError> {
        self.stock_db.delete(symbol.to_owned())
    }

    // Returns a vector with all the stored stocks
    pub fn get_stored_stocks(&self)  -> Result<Vec<Stock>, PersistanceError> {
        unimplemented!();
    }

    // Updates the price of the stored stocks, and returns them in a vector
    pub fn update_stocks(&self) ->Result<Vec<Stock>, PersistanceError> {
        unimplemented!();
    }

    // Returns a list of all the available stocks in the API
    pub async fn get_available_stocks(&self) -> Result<Vec<Stock>, Box<dyn std::error::Error+Sync+Send>>{
        match self.stock_api.get_stock_list().await {
            Ok(stocks) => {
                Ok(stocks
                    .iter()
                    .map(Stock::from)
                    .collect::<Vec<Stock>>())
            },
            Err(e) => Err(e)
        }
    }

    // Returns a list of all the markets available in the API
    pub fn get_available_markets() -> Result<Vec<Market>, PersistanceError> {
        unimplemented!();
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