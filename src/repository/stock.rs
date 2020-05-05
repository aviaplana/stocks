pub mod stock_sqlite;
pub mod stock_api;

use stock_api::StockListElement;

#[derive(Debug, PartialEq, Clone)]
pub struct Stock {
    symbol: String,
    name: String,
    price: f32,
    initial_price: f32,
    market: u16 // Usar id o posar struct Market?
}

impl Stock {
    pub fn set_price(&mut self, price: f32) {
        self.price = price;
    }
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