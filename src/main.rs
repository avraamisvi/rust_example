#[macro_use] extern crate serde_derive;

extern crate crest;
extern crate serde;

use crest::error::Result;
use crest::prelude::*;

#[derive(Debug, Deserialize)]
struct Ticker {
    high: String,
    low: String,
    vol: String,
    last: String,
    buy: String,
    sell: String,
    date: u32
}

#[derive(Debug, Deserialize)]
struct ResponseTicker {
    ticker: Ticker 
}

fn example() -> Result<ResponseTicker> {
    let endpoint = Endpoint::new("https://www.mercadobitcoin.net/")?;
    let request = endpoint.get(&["api/BTC/ticker"])?;
    let response = request.send()?;
    let resp = response.into::<ResponseTicker>()?;
    Ok(resp)
}

fn main() {
    let resp = example();
    
    match resp {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("Some error: {:?}", e),
    }
}