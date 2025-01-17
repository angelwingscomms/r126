use r126::gate::send;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tokio::spawn(async move {
        println!("move");
        let bought = false;
        while !bought {
            println!("buy");
            
            match send(
                "spot/orders",
                &serde_json::json!({
                    "currency_pair": "NC_USDT",
                    "type": "market",
                    "side": "buy",
                    "account": "spot",
                    "amount": 1.44,
                }),
            )
            .await
            {
                Ok(r) => println!("buy res: {:#?}", r),
                Err(e) => println!("buy error: {:#?}", e),
            };
        }
    })
    .await?;

    dotenv::dotenv().ok();
    Ok(())
}
