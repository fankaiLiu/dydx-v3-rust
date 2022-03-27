use dydx_v3_rust::{entities::market::Makret, Client};
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
            .get_markets(&json!({}))
            .await
            .unwrap()
            .response
            .text()
            .await
            .unwrap();
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
}

fn json_to_map(s: &str) -> Value {
    let mut s = s.to_string();
    s.retain(|c| !c.is_ascii_control());
    let v: Value = serde_json::from_str(&s).unwrap();
    v
}
