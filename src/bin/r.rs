use r126::gate::util::{buy, sell};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    loop {
        let buy_res = buy("ARTELA", 9.0).await?;
        println!("buy res: {:#?}", buy_res);
        if let Ok(f) = buy_res["filled_amount"]
            .as_str()
            .ok_or("filled_amount as_str")?
            .parse::<f64>()
        {
            let sell_res = sell("ARTELA", f).await?;
            println!("sell res: {:#?}", sell_res);
            break;
        }
    }
    Ok(())
}
