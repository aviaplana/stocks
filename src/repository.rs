pub mod market;
pub mod stock;
pub mod error;

use hyper_tls::HttpsConnector;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use error::PersistanceError;
use stock::Stock;
use market::Market;

pub type DbConn = PooledConnection<SqliteConnectionManager>;
pub type HttpClient = hyper::Client<HttpsConnector<hyper::client::HttpConnector>, hyper::Body>;

// Stores a stock in the local storage
pub fn add_stock(db_conn: &DbConn, stock: &Stock) -> Result<(), PersistanceError> {
    stock::stock_db::add(db_conn, stock)
}

// Deletes a stock from the local storage
pub fn delete_stock(db_conn: &DbConn, symbol: &str) -> Result<(), PersistanceError> {
    stock::stock_db::delete(db_conn, symbol)
}

// Returns a vector with all the stored stocks
pub fn get_stored_stocks(db_conn: &DbConn)  -> Result<Vec<Stock>, PersistanceError> {
    stock::stock_db::get_all(db_conn)
}

// Updates the price of a stored stock.
pub fn update_price(db_conn: &DbConn, symbol: &str, price: f32) -> Result<(), PersistanceError> {
    stock::stock_db::update_price(db_conn, symbol, price)
}

// Get the current price of a stock
pub async fn get_current_price(client: &HttpClient, symbol: &str) ->Result<f32, Box<dyn std::error::Error+Sync+Send>> {
    match stock::stock_api::get_stock_price(client, symbol).await {
        Ok(stock_price) => Ok(stock_price.price),
        Err(e) => Err(e)
    }
}

// Returns a list of all the available stocks in the API
pub async fn get_available_stocks(client: &HttpClient, ) -> Result<Vec<Stock>, Box<dyn std::error::Error+Sync+Send>>{
    match stock::stock_api::get_stock_list(client).await {
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