mod stocks;
mod persistance;

use stocks::*;
use persistance::*;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let persistance = StockRepository::new();

    println!("Stored: ");
    for entry in persistance.get_entries() {
        println!("{:?}", entry);
    }

    let stocks_repo = StockRepo::new();
    stocks_repo.get_stock_price("T").await.unwrap();
    Ok(())
}


