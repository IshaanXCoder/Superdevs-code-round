use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use axum::{Router,Json, routing::get, extract::Path};
use serde_json::json;
use std::net::SocketAddr;
// use hyper::Server;



async fn root_handler() -> Json<serde_json::Value> {
    // json! macro → lets you write JSON-like syntax easily
    Json(json!({
        "message": "Hello, World!"
    }))
}

async fn balance_handler(Path(pubkey): Path<String>) -> Json<serde_json::Value> {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let pubkey = pubkey.parse::<Pubkey>().unwrap(); // or handle error
    let lamports = client.get_balance(&pubkey).unwrap();
    let sol = lamports as f64 / 1_000_000_000.0;
    Json(json!({
        "balance": sol
    }))
}

async fn airdrop_handler(Path(pubkey): Path<String>) -> Json<serde_json::Value> {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let pubkey = pubkey.parse::<Pubkey>().unwrap(); // or handle error
    let signature = client.request_airdrop(&pubkey, 1000000000).unwrap();

    Json(json!({
        "status": "success",
        "signature": signature.to_string()
      }))
}

async fn details_handler(Path(pubkey): Path<String>) -> Json<serde_json::Value> {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let pubkey = pubkey.parse::<Pubkey>().unwrap(); // or handle error
    let balance = client.get_balance(&pubkey).unwrap();
    let sol = balance as f64 / 1_000_000_000.0;

    Json(json!({
        "balance": sol,
        "pubkey": pubkey.to_string(),
        
    }))
}


#[tokio::main]
//this macro is used to mark the main function as async, warna normally main fn cannot by async
async fn main() {

    let app = Router::new()
    .route("/", get(root_handler))
    //app ko ek new route banaya, an empty route
    .route("/balance/:pubkey", get(balance_handler))
    .route("/airdrop/:pubkey", get(airdrop_handler))
    .route("/details/:pubkey", get(details_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    //Listen locally on port 8080
    
    println!("Server is running on http://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}