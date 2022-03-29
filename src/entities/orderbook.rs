use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orderbook {
    pub market: String,
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order
{
    pub price: String,
    pub size: String,
}
impl Orderbook {
    pub fn new(market:&str) -> Orderbook {
        Orderbook {
            bids: Vec::new(),
            market: market.to_string(),
            asks: Vec::new(),
        }
    }
}