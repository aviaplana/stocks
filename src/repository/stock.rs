pub mod stock_db;
pub mod stock_api;

use log::debug;
use stock_api::StockListElement;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Position {
    pub stock: Stock,
    pub initial_price: f32,
}

impl From<&str> for Position {
    fn from(json: &str) -> Self {
        debug!(target: "stock_position", "Deserializing {}", json);
        serde_json::from_slice(json.as_bytes()).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub price: f32,
    pub market: u16 // Usar id o posar struct Market?
}

impl From<&str> for Stock {
    fn from(json: &str) -> Self {
        debug!(target: "stock", "Deserializing {}", json);
        serde_json::from_slice(json.as_bytes()).unwrap()
    }
}

impl From<&StockListElement> for Stock {
    fn from(stock: &StockListElement) -> Self {
        Stock {
            symbol: stock.symbol.to_owned(),
            price: stock.price,
            name: stock.name.to_owned(),
            market: 0,
        }
    }
}