use std::time::Duration;

use r126::gate::{buy, sell, send};
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    // buy("CAM", 3.0).await;
    sell("CAM", 144.0).await;
    
    // tokio::spawn(async move {
    //     // let bought = false;
    //     let mut count = 0;
    //     let start = Instant::now();
    //     while start.elapsed() < Duration::from_secs(1) {
    //         buy("CAM", 1.44).await;
    //         count += 1;
    //     }
    //     println!("buy count: {count}");
    //     println!("buy time: {}", start.elapsed().as_secs());
    // })
    // .await?;

    // tokio::spawn(async move {
    //     let sold = false;
    //     // let mut count = 0;
    //     // let start = Instant::now();
    //     while !sold {
    //         // while start.elapsed() < Duration::from_secs(1) {
    //         sell("CAM", 1.44).await;
    //         // count += 1;
    //     }
    //     // println!("sell count: {count}");
    //     // println!("sell time: {}", start.elapsed().as_secs());
    // })
    // .await?;
    Ok(())
}
