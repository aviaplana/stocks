extern crate hyper;
use hyper::{
    Client, 
    Body,
    body::to_bytes,
    Uri,
    client::HttpConnector,
};
use hyper_tls::HttpsConnector;
use serde_json;
use serde::Deserialize;

enum Endpoint {
    RealTimePrice(String),
    StockList,
}

impl Endpoint {
    const BASE_URL: &'static str = "https://financialmodelingprep.com/";

    pub fn to_uri(&self) -> Uri {
        let route = match self {
            Self::RealTimePrice(args) => "api/v3/stock/real-time-price/".to_owned() + args,
            Self::StockList => "api/v3/company/stock/list".into()
        };

        let base = String::from(Self::BASE_URL);

        format!("{}{}", base, route).parse().unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct StockListElement {
    pub symbol: String,
    #[serde(default)]
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Deserialize)]
pub struct StockPriceResponse {
    pub symbol: String,
    pub price: f32,
}

#[derive(Deserialize)]
pub struct StocksListResponse {
    #[serde(alias = "symbolsList")] 
    stocks: Vec<StockListElement>,
}

pub struct StockApi { 
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl StockApi {
    pub fn new() -> StockApi {
        let https = HttpsConnector::new();
        StockApi {
            client: Client::builder().build::<_, hyper::Body>(https)
        }
    }

    pub async fn get_stock_list(&self) -> Result<Vec<StockListElement>, Box<dyn std::error::Error + Send + Sync>> {        
        let uri = Endpoint::StockList.to_uri();
        let response = self.client.get(uri).await;
        match response {
            Ok(mut resp) => {
                match to_bytes(resp.body_mut()).await {
                    Ok(body) => {
                        let stock_price: StocksListResponse = serde_json::from_slice(&body).unwrap();
                        //println!("{:?}", stock_price);
                        Ok(stock_price.stocks)
                    },
                    Err(e) => Err(e.into())
                }
            },
            Err(e) => Err(e.into()),
        }
    }

    pub async fn get_stock_price(&self, symbol: &str) -> Result<StockPriceResponse, Box<dyn std::error::Error + Send + Sync>> {        
        let uri = Endpoint::RealTimePrice(symbol.into()).to_uri();
        let response = self.client.get(uri).await;

        match response {
            Ok(mut resp) => {
                match to_bytes(resp.body_mut()).await {
                    Ok(body) => {
                        let stock_price: StockPriceResponse = serde_json::from_slice(&body).unwrap();
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
