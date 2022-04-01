use chrono::Utc;
use dydx_v3_rust::{
    entities::{
        liquidity_providers::{LiquidityProviders, Liquidity},
        market::Makret,
        orderbook::{Order, Orderbook},
        trade::{Trade, Trades}, stats::Stats,
    },
    Client,
};
use serde::Deserialize;
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
        let response = self.client.get_markets(&json!({})).await.unwrap();
        let mut result = Vec::new();
        let markets = json_to_map(&response);
        if markets.is_object() {
            let markets = markets.as_object().unwrap();
            for (_, v) in markets {
                if v.is_object() {
                    let markets = v.as_object().unwrap();
                    for (_, v) in markets {
                        let makret = Makret::deserialize(v).unwrap();
                        result.push(makret);
                    }
                    return Some(result);
                }
            }
        }
        None
    }
    pub async fn get_orderbook(&self, market: &str) -> Option<Orderbook> {
        let response = self.client.get_orderbook(market).await.unwrap();
        let mut result = Orderbook::new(market);
        let orderbook = json_to_map(&response);
        if orderbook.is_object() {
            let orderbook = orderbook.as_object().unwrap();
            for (k, v) in orderbook {
                if k == "bids" {
                    let bids = v.as_array().unwrap();
                    for v in bids {
                        let order = Order::deserialize(v).unwrap();
                        result.bids.push(order);
                    }
                } else if k == "asks" {
                    let asks = v.as_array().unwrap();
                    for v in asks {
                        let order = Order::deserialize(v).unwrap();
                        result.asks.push(order);
                    }
                }
            }
            return Some(result);
        }
        None
    }

    pub async fn get_trades(&self, market: &str) -> Option<Trades> {
        let response = self.client.get_trades(market).await.unwrap();
        let mut result = Trades::new(market);
        let trade = json_to_map(&response);
        if trade.is_object() {
            let trade = trade.as_object().unwrap();
            for (k, v) in trade {
                if k == "trades" {
                    let trades = v.as_array().unwrap();
                    for v in trades {
                        let trade = value_to_trade(&v);
                        if trade.is_some() {
                            result.push(trade.unwrap());
                        }
                    }
                }
            }
            return Some(result);
        }
        None
    }

    pub async fn get_liquidity_providers(
        &self,
        parameters: &Option<Value>,
    ) -> Option<LiquidityProviders> {
        let response = self
            .client
            .get_liquidity_providers(parameters)
            .await
            .unwrap();
        let mut result = LiquidityProviders::default();
        let liquidity_providers = json_to_map(&response);
        if liquidity_providers.is_object() {
            let liquidity_providers = liquidity_providers.as_object().unwrap();
            for (k, v) in liquidity_providers {
                if k == "liquidityProviders" {
                    let liquidity_providers = v.as_object().unwrap();
                    for (k, v) in liquidity_providers {
                        result.position_id=k.to_string();
                        result.liquidity = Liquidity::deserialize(v).unwrap();
                    }
                }
            }
            return Some(result);
        }
        None
    }

    pub async fn get_market_stats(&self, market: &str, parameters: &Option<Value>) -> Option<Stats> {
        let response = self.client.get_market_stats(market,&parameters).await.unwrap();
        let market_stats = json_to_map(&response);
        if market_stats.is_object() {
            let market_stats = market_stats.as_object().unwrap();
            let market_stats=market_stats.get("markets").unwrap();
            for (k, v) in market_stats.as_object().unwrap() {
                if k == market {
                    let stats=Stats::deserialize(v).unwrap();
                    return Some(stats);
                }
            }
        }
        None
    }
}
pub fn value_to_trade(value: &Value) -> Option<Trade> {
    if value.is_object() {
        let trade = value.as_object().unwrap();
        let (side, size, price, created_at) = (
            trade.get("side").unwrap().as_str().unwrap(),
            trade
                .get("size")
                .unwrap()
                .as_str()
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            trade
                .get("price")
                .unwrap()
                .as_str()
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            chrono::DateTime::parse_from_rfc3339(trade.get("createdAt").unwrap().as_str().unwrap())
                .unwrap()
                .with_timezone(&Utc),
        );
        let trade = Trade::new(side, size, price, created_at);
        return Some(trade);
    }
    None
}

fn json_to_map(s: &str) -> Value {
    let mut s = s.to_string();
    s.retain(|c| !c.is_ascii_control());
    let v: Value = serde_json::from_str(&s).unwrap();
    v
}
