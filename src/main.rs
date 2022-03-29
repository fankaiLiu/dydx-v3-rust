use crate::services::market_service::MakretService;
mod services{pub mod market_service;}
#[tokio::main]
async fn main() 
{
    //let  client = Client::new("production");
    let services=MakretService::new();
    let makrets =services.get_markets().await;
    // println!("{:?}",makrets);
    for makret in makrets.unwrap() {
       let order_book= services.get_orderbook(&makret.market).await;
       println!("{:?}",order_book);
    }
}
