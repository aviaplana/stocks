extern crate hyper;
use hyper::{
    Client, 
    Body,
    body::to_bytes,
    client::HttpConnector,
};
use hyper_tls::HttpsConnector;
use serde_json;
use serde::Deserialize;

const BASE_URL: &'static str = "https://financialmodelingprep.com/";
const REAL_TIME_PRICE: &'static str = "api/v3/stock/real-time-price/";

#[derive(Debug, Deserialize)]
pub struct StockPrice {
    symbol: String,
    price: f32,
}

pub struct StockRepo { 
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl StockRepo {
    pub fn new() -> StockRepo {
        let https = HttpsConnector::new();
        StockRepo {
            client: Client::builder().build::<_, hyper::Body>(https)
        }
    }

    pub async fn get_stock_price(&self, symbol: &str) -> Result<StockPrice, Box<dyn std::error::Error + Send + Sync>> {        
        let uri = format!("{}{}{}", BASE_URL, REAL_TIME_PRICE, symbol).parse().unwrap();
    
        let response = self.client.get(uri).await;

        match response {
            Ok(mut resp) => {
                match to_bytes(resp.body_mut()).await {
                    Ok(body) => {
                        let stock_price: StockPrice = serde_json::from_slice(&body).unwrap();
                        println!("{:?}", stock_price);
                        Ok(stock_price)
                    },
                    Err(e) => Err(e.into())
                }
            },
            Err(e) => Err(e.into()),
        }
    }
}
