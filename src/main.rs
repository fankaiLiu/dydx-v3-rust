use serde_json::json;

use crate::services::market_service::MakretService;
mod services {
    pub mod market_service;
}
#[tokio::main]
async fn main() {
    //let  client = Client::new("production");
    let services = MakretService::new();
    let makrets = services.get_markets().await;
    //println!("{:?}",makrets);
    // for makret in makrets.unwrap() {
    //    let order_book= services.get_orderbook(&makret.market).await;
    //    println!("{:?}",order_book);
    // }

    // for makret in makrets.unwrap() {
    //     let trades = services.get_trades(&makret.market).await;
    //     println!("{:?}", trades);
    // }
    // let trades = services.get_liquidity_providers(&None).await;
    // if let Some(trades) = trades {
    //     println!("{:?}", trades);
    // }
    for makret in makrets.unwrap() {
        let value=json!({
            "days": 7,
        }); 
        let stats = services.get_market_stats(&makret.market, &Some(value)).await;
        dbg!(stats);
    }
}
