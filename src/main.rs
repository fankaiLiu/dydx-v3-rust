use crate::services::market_service::MakretService;
mod services{pub mod market_service;}
#[tokio::main]
async fn main() 
{
    //let  client = Client::new("production");
    let services=MakretService::new();
    let makrets =services.get_markets().await;
    println!("{:?}",makrets);
}
