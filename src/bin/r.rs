use r126::gate::util::{buy, sell};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    loop {
        let buy_res = buy("ARTELA", 9.0).await?;
        println!("buy res: {:#?}", buy_res);
        if let Some(f) = buy_res["filled_amount"].as_str() {
            let sell_res = sell("ARTELA", f).await?;
            println!("sell res: {:#?}", sell_res);
            break;
        }
    }
    Ok(())
}
