use super::send;

pub async fn sell(t: &str, amount: f64) -> anyhow::Result<serde_json::Value> {
    Ok(send(
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
    .await?)
}

pub async fn buy(t: &str, amount: f64) -> anyhow::Result<serde_json::Value> {
    Ok(send(
        "spot/orders",
        &serde_json::json!({
            "currency_pair": t.to_owned() + "_USDT",
            "type": "market",
            "side": "buy",
            "time_in_force": "ioc",
            "account": "spot",
            "amount": amount,
        }),
    )
    .await?)
}
