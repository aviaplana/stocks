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
    let list_stocks = stocks_repo.get_stock_list().await.unwrap();
    for stock in list_stocks {
        println!("{}({}): {}", stock.name, stock.symbol, stock.price);
    }
    Ok(())
}


