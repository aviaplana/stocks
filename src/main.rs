mod repository;

use repository::{
    StockRepository,
    stock::Stock,
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let repository = StockRepository::new().unwrap();

    println!("Stored stocks: ");
    for stock in repository.get_stored_stocks().unwrap() {
        println!("{} ({}) - {}$", stock.symbol, stock.name, stock.price);
        let price = repository.get_current_price(&stock.symbol).await.unwrap();
        repository.update_price(&stock.symbol, price).unwrap();
    }

    repository.delete_stock("AAPL").unwrap();
    
    println!("Stored stocks: ");
    for stock in repository.get_stored_stocks().unwrap() {
        println!("{} ({}) - {}$", stock.symbol, stock.name, stock.price);
    }

    Ok(())
}




