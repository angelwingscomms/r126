use r126::gate::send;
use reqwest::Client;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use dotenv::dotenv;

// #[derive(Deserialize, Debug)]
// struct OrderResponse {
//     result: bool,
//     order_id: String,
// }

// #[derive(Deserialize)]
// struct TickerResponse {
//     result: bool,
//     last: f64,
// }

async fn buy_token(
    client: &Client,
    success: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    while !success.load(Ordering::SeqCst) {
        let res = send("spots/orders", &serde_json::json!({
            "currency_pair": "LILY_USDT",
            "type": "market",
            "side": "buy",
            "account": "spot",
            "amount": 1,
        })).await?;();

        println!("res: {:#?}", res);
        break;

        // if res.result {
        //     success.store(true, Ordering::SeqCst);
        //     return Ok(res.result);
        // }
    }

    Ok(())
}

// async fn monitor_price(client: &Client, purchase_price: f64, success: Arc<AtomicBool>) -> anyhow::Result<()> {
//     while success.load(Ordering::SeqCst) {
//         sleep(Duration::from_secs(9)).await;

//         let res: TickerResponse = client
//             .get("https://api.gateio.ws/api/v4/spot/tickers")
//             .query(&[("currency_pair", "LILY_USDT")])
//             .send()
//             .await?
//             .json()
//             .await?;

//         if res.result {
//             let current_price = res.last;
//             if current_price < purchase_price * 0.91 || detect_downtrend().await {
//                 client
//                     .post("https://api.gateio.ws/api/v4/spots/orders")
//                     .json(&serde_json::json!({
//                         "currency_pair": "LILY_USDT",
//                         "type": "sell",
//                         "account": "spot",
//                         "amount": 1 // Adjust amount as needed
//                     }))
//                     .send()
//                     .await?;
//                 break;
//             }
//         }
//     }
// }

// async fn detect_downtrend(
//     client: &Client,
//     currency_pair: &str,
//     checks: usize,
// ) -> Result<bool, Box<dyn std::error::Error>> {
//     let mut prices = Vec::new();
//     for _ in 0..checks {
//         let res: TickerResponse = client
//             .get("https://api.gateio.ws/api/v4/spot/tickers")
//             .query(&[("currency_pair", currency_pair)])
//             .send()
//             .await?
//             .json()
//             .await?;
//         if res.result {
//             prices.push(res.last);
//         }
//         sleep(Duration::from_secs(9)).await;
//     }
//     for i in 1..prices.len() {
//         if prices[i] >= prices[i - 1] {
//             return Ok(false);
//         }
//     }
//     Ok(true)
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let client = Client::new();
    let success = Arc::new(AtomicBool::new(false));

    buy_token(&client, Arc::clone(&success)).await?;
    // println!("purchase price: {purchase_price}");
    // monitor_price(&client, purchase_price, Arc::clone(&success)).await;

    Ok(())
}
