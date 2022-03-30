use chrono::DateTime;

#[derive( Debug, Clone, PartialEq)]
pub struct Trades {
    pub market: String,
    pub buy: Vec<Trade>,
    pub sell: Vec<Trade>,
}

impl Trades {
    pub fn new(market: &str) -> Trades {
        Trades {
            market: market.to_string(),
            buy: Vec::new(),
            sell: Vec::new(),
        }
    }

    pub fn push(&mut self, trade: Trade) {
        if trade.side == Side::Buy {
            self.buy.push(trade);
        } else {
            self.sell.push(trade);
        }
    }
}

#[derive( Debug, Clone, PartialEq)]
pub struct Trade {
    pub side: Side,
    pub size: f64,
    pub price: f64,
    pub created_at: DateTime<chrono::Utc>,
}
impl Trade {
    pub fn new(side: &str, size: f64, price: f64, created_at: DateTime<chrono::Utc>) -> Trade {
        Trade {
            side: Side::new(side),
            size,
            price,
            created_at,
        }
    }
}

#[derive( Debug, Clone, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    pub fn new(side: &str) -> Side {
        match side {
            "BUY" => Side::Buy,
            "SELL" => Side::Sell,
            _ => Side::Buy,
        }
    }
}
