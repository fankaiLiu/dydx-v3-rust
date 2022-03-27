
use dydx_v3_rust::Client;
use serde_json::json;
#[tokio::main]
async fn main() 
{
    let  client = Client::new("production");
    let response = client.get_orderbook(&json!({"market":"EOS-USD"})).await.unwrap();
    println!("{:?}", response.response.text().await.unwrap());
}
