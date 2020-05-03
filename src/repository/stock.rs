pub mod stock_sqlite;

#[derive(Debug, PartialEq, Clone)]
pub struct Stock {
    symbol: String,
    price: f32,
    initial_price: f32,
    market: u16 // Usar id o posar struct Market?
}

impl Stock {
    pub fn set_price(&mut self, price: f32) {
        self.price = price;
    }
}