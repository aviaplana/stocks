mod repository;
mod server;
use r2d2::{
    Pool,
};
use r2d2_sqlite::{
    SqliteConnectionManager
};
use std::thread;
use tokio;

#[derive(Debug, PartialEq)]
pub enum Operation {
    ListStored,
    ListAvailable,
    UpdatePrices,
    Delete(String),
    Error
}

impl From<String> for Operation {
    fn from(val: String) -> Operation {
        println!("-{}-", val);
        println!("{}", val == "list_available");

        match val.as_str() {
            "list_stored" => Self::ListStored,
            "list_available" => Self::ListAvailable,
            "update_prices" => Self::UpdatePrices,
            op if op.starts_with("delete") => {
                let parts = op.split(' ').collect::<Vec<&str>>();
                let stock = parts.get(1).unwrap();
                Self::Delete(String::from(*stock))
            },
            _ => Self::Error
        }
    }
}

fn get_db_pool_connection() -> Pool<SqliteConnectionManager>{
    let manager = SqliteConnectionManager::file("stocks.db");
    r2d2::Pool::new(manager).expect("Couldn't create pool.")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db_pool = get_db_pool_connection();
    let rx_ch = server::launch_tcp_server();

    loop {
        let mut received_str = rx_ch.recv().unwrap();
        received_str.pop(); // Pop removes tailing new line.
        let operation = Operation::from(received_str);
        
        println!("Channel got : {:?}", operation);
        let pool = db_pool.clone();

        match operation {
            Operation::ListAvailable => {
                thread::spawn(move || {
                    let connection = pool.get().unwrap();
                    repository::get_stored_stocks(&connection);
                });
            },
            _ => {}
        }

    }
    Ok(())
}

#[test]
fn test_str_to_operation_list_available() {
    let raw = "list_available".to_owned();
    let operation = Operation::from(raw);
    assert_eq!(Operation::ListAvailable, operation);
}

#[test]
fn test_str_to_operation_error() {
    let raw = "fail".to_owned();
    let operation = Operation::from(raw);
    assert_eq!(Operation::Error, operation);
}