use dydx_v3_rust::{entities::{market::Makret, orderbook::{Orderbook, Order}}, Client};
use serde::{Deserialize};
use serde_json::{json, Value};

pub struct MakretService {
    client: Client,
}
impl MakretService {
    pub fn new() -> MakretService {
        MakretService {
            client: Client::new("production"),
        }
    }
    pub async fn get_markets(&self) -> Option<Vec<Makret>> {
        let response = self
            .client
            .get_markets(&json!({})).await.unwrap();
        let mut  result=Vec::new();
        let markets = json_to_map(&response);
        if markets.is_object() {
            let markets = markets.as_object().unwrap();
            for (_, v) in markets {
                if v.is_object(){
                    let markets = v.as_object().unwrap();
                    for (_, v) in markets {
                        let makret=Makret::deserialize(v).unwrap();
                        result.push(makret);
                    }
                  return  Some(result);
                }
            }
        }
        None
    }
    pub async fn get_orderbook(&self, market: &str) -> Option<Orderbook> {
        let response = self
            .client
            .get_orderbook(market).await.unwrap();
        let mut result =Orderbook::new(market);
        let orderbook = json_to_map(&response);
        if orderbook.is_object() {
            let orderbook = orderbook.as_object().unwrap();
            for (k, v) in orderbook {
               if k=="bids"{
                    let bids = v.as_array().unwrap();
                    for v in bids {
                        let order=Order::deserialize(v).unwrap();
                        result.bids.push(order);
                    }
                }
                else if k=="asks"{
                    let asks = v.as_array().unwrap();
                    for v in asks {
                        let order=Order::deserialize(v).unwrap();
                        result.asks.push(order);
                    }
                }
               }
               return Some(result);
            }
        None
    }
}

fn json_to_map(s: &str) -> Value {
    let mut s = s.to_string();
    s.retain(|c| !c.is_ascii_control());
    let v: Value = serde_json::from_str(&s).unwrap();
    v
}
