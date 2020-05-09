pub mod stock_db;
pub mod stock_api;

use stock_api::StockListElement;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub price: f32,
    pub initial_price: f32,
    pub market: u16 // Usar id o posar struct Market?
}

impl From<&StockListElement> for Stock {
    fn from(stock: &StockListElement) -> Self {
        Stock {
            symbol: stock.symbol.to_owned(),
            price: stock.price,
            name: stock.name.to_owned(),
            initial_price: 0.0,
            market: 0,
        }
    }
}