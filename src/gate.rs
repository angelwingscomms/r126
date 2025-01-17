use serde::Serialize;
use serde_json::json;
use sha2::Digest;
use sha2::Sha512;
use std::env;

use hmac::{Hmac, Mac};
use hex::encode;
use std::time::{SystemTime, UNIX_EPOCH};

// Define type alias for Hmac-Sha512
type HmacSha512 = Hmac<Sha512>;

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
    println!("send");
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
    println!("here");

    // Generate payload hash
    let payload_hash = encode(Sha512::digest(payload.as_bytes()));
    println!("here2");

    // Generate the signature
    let signature = generate_signature(
        method,
        &url,
        query_string,
        &payload_hash,
        &timestamp,
        &secret,
    );
    println!("here3");

    // Create request client and set headers
    let client = reqwest::Client::new();
    let response = client
        .post(format!("https://api.gateio.ws{url}"))
        .header("KEY", key)
        .header("Timestamp", &timestamp)
        .header("SIGN", &signature)
        .body(payload)
        .send()
        .await?
        .json()
        .await?;
    println!("here4");
    
    println!("response: {:#?}", response);

    Ok(response)
}
