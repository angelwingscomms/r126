use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use sha2::Digest;
use sha2::Sha512;
use std::env;

use hex::encode;
use hmac::{Hmac, Mac};
use std::time::{SystemTime, UNIX_EPOCH};

// Define type alias for Hmac-Sha512
type HmacSha512 = Hmac<Sha512>;

enum TradeStatus {
    Untradable,
    Buyable,
    Sellable,
    Tradable,
}

#[derive(Serialize, Deserialize)]
struct Pair {
    id: Option<String>,
    base: Option<String>,
    quote: Option<String>,
    fee: Option<String>,
    min_base_amount: Option<String>,
    min_quote_amount: Option<String>,
    max_base_amount: Option<String>,
    max_quote_amount: Option<String>,
    amount_precision: Option<u32>,
    precision: Option<u32>,
    trade_status: Option<TradeStatus>,
    sell_start: Option<i64>,
    buy_start: Option<i64>,
}

fn generate_signature(
    method: &str,
    url: &str,
    query_string: &str,
    payload: &str,
    timestamp: &str,
    secret: &str,
) -> String {
    // Concatenate signature string
    let signature_string = format!(
        "{}\n{}\n{}\n{}\n{}",
        method, url, query_string, payload, timestamp
    );

    // Create HMAC-Sha512 instance with the secret key
    let mut mac =
        HmacSha512::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");

    // Input the signature string to HMAC-Sha512 instance
    mac.update(signature_string.as_bytes());

    // Get the resulting HMAC digest
    let result = mac.finalize();

    // Encode the HMAC digest to hex string
    encode(result.into_bytes())
}

pub async fn send<T: Serialize>(path: &str, payload: &T) -> anyhow::Result<serde_json::Value> {
    let key = env::var("GATE")?;
    let secret = env::var("GATE_SECRET")?;
    let method = "POST";
    let url = format!("/api/v4/{path}");
    let query_string = "";
    let payload = serde_json::to_string(payload)?; // Payload should be hex-encoded SHA512 hash of the request body
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        .to_string();

    // Generate payload hash
    let payload_hash = encode(Sha512::digest(payload.as_bytes()));

    // Generate the signature
    let signature = generate_signature(
        method,
        &url,
        query_string,
        &payload_hash,
        &timestamp,
        &secret,
    );

    // Create request client and set headers
    let client = reqwest::Client::new();
    let response = client
        .post(format!("https://api.gateio.ws{url}"))
        .header("KEY", key)
        .header("Content-Type", "application/json")
        .header("Timestamp", &timestamp)
        .header("SIGN", &signature)
        .body(payload)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn send_get(path: &str) -> anyhow::Result<serde_json::Value> {
    let key = env::var("GATE")?;
    let secret = env::var("GATE_SECRET")?;
    let method = "POST";
    let url = format!("/api/v4/{path}");
    let query_string = "";
    let payload = ""; // Payload should be hex-encoded SHA512 hash of the request body
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        .to_string();

    // Generate payload hash
    let payload_hash = encode(Sha512::digest(payload.as_bytes()));

    // Generate the signature
    let signature = generate_signature(
        method,
        &url,
        query_string,
        &payload_hash,
        &timestamp,
        &secret,
    );

    // Create request client and set headers
    let client = reqwest::Client::new();
    let response = client
        .post(format!("https://api.gateio.ws{url}"))
        .header("KEY", key)
        .header("Content-Type", "application/json")
        .header("Timestamp", &timestamp)
        .header("SIGN", &signature)
        .body(payload)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn sell(t: &str, amount: f64) {
    match send(
        "spot/orders",
        &serde_json::json!({
            "currency_pair": t.to_owned() + "_USDT",
            "type": "market",
            "side": "sell",
            "time_in_force": "fok",
            "account": "spot",
            "amount": amount,
        }),
    )
    .await
    {
        Ok(r) => {
            println!("sold {amount} {t}. res: {:#?}", r);
            // sold = true;
        }
        Err(e) => println!("sell error: {:#?}", e),
    };
}

pub async fn buy(t: &str, amount: f64) {
    match send(
        "spot/orders",
        &serde_json::json!({
            "currency_pair": "SOL_USDT",
            "type": "market",
            "side": "buy",
            "time_in_force": "ioc",
            "account": "spot",
            "amount": amount,
            // "price":  0.144
        }),
    )
    .await
    {
        Ok(r) => {
            println!("bought {amount} {t}. res: {:#?}", r);
            // bought = true;
        }
        Err(e) => println!("buy error: {:#?}", e),
    };
}

pub async fn pairs() -> anyhow::Result<Vec<Pair>> {
    Ok(serde_json::from_value::<Vec<Pair>>(
        send_get("/spot/currency_pairs").await?,
    )?)
}
