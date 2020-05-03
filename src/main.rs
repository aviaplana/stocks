mod repository;
mod stocks_api;

use repository::{
    StockRepository
};
use stocks_api::StockApi;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let persistance = StockRepository::new().unwrap();

    println!("Stored: ");
    /*for entry in persistance.get_entries() {
        println!("{:?}", entry);
    }
*/
    let stocks_repo = StockApi::new();
    stocks_repo.get_stock_price("T").await.unwrap();
    Ok(())
}


