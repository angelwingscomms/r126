use anyhow::anyhow;
use chrono::{DateTime, NaiveDateTime};
use reqwest::Client;
use serde::Deserialize;
use tokio::time::{sleep, Duration, Instant};

#[derive(Deserialize, Clone, Debug)]
struct OHLC {
    time: String,
    close: f64,
    low: f64,
    high: f64,
}

type ResOHLC = Vec<String>;

impl TryFrom<ResOHLC> for OHLC {
    type Error = anyhow::Error;
    fn try_from(value: ResOHLC) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(anyhow::anyhow!("Expected exactly 8 elements in ResOHLC"));
        }
        let close = value[2]
            .parse()
            .map_err(|_| anyhow::anyhow!("failed parse - 2"))?;
        let high = value[3]
            .parse()
            .map_err(|_| anyhow::anyhow!("failed parse - 3"))?;
        let low = value[4]
            .parse()
            .map_err(|_| anyhow::anyhow!("failed parse - 4"))?;
        let naive_datetime = DateTime::from_timestamp(
            value[0]
                .parse()
                .map_err(|_| anyhow::anyhow!("failed parse - time"))?,
            0,
        )
        .ok_or(anyhow!("failed to get time from timestamp in OHLC"))?;
        // Format to HH:MM:SS
        Ok(OHLC {
            close,
            low,
            high,
            time: naive_datetime.format("%H:%M:%S").to_string(),
        })
    }
}

impl TryFrom<&Vec<String>> for OHLC {
    type Error = anyhow::Error;
    fn try_from(value: &Vec<String>) -> Result<Self, Self::Error> {
        OHLC::try_from(value.clone())
    }
}

async fn get_ohlc(client: &Client, symbol: &str) -> Result<Vec<OHLC>, anyhow::Error> {
    let url = format!(
        "https://api.gateio.ws/api/v4/spot/candlesticks?currency_pair={}&interval=3m&limit=2",
        symbol
    ); // Use 3-minute interval
    println!("Fetching OHLC data from URL: {}", url);
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<Vec<ResOHLC>>()
        .await?;
    println!("fr");
    println!("Fetched OHLC data: {:?}", response);
    let all = response
        .iter()
        .map(|s| OHLC::try_from(s))
        .collect::<anyhow::Result<Vec<OHLC>>>()?;
    Ok(all)
}

fn check_trend(ohlc: &[OHLC]) -> bool {
    let first = &ohlc[0];
    let second = &ohlc[1];

    let gap_first = first.close - first.low;
    let gap_second = second.close - second.low;

    let range_first = first.high - first.low;
    let range_second = second.high - second.low;

    // Rule 1
    let rule1 = gap_second > gap_first * 1.008; // 10% more significant

    // Rule 2
    let rule2 = first.close > first.low * 1.05 && second.close > second.low * 1.05; // 5% above the low

    // Rule 3
    let rule3 = range_second > range_first * 1.1; // 10% more significant

    let result = rule1 && /*rule2 &&*/ rule3;
    result
}

async fn execute_trade(
    client: &Client,
    symbol: &str,
    buy_price: f64,
) -> Result<(), reqwest::Error> {
    println!("Pretend buying at price: {}", buy_price);
    let amount_bought = 9.0 / buy_price; // Calculate amount bought with $9
    println!("Amount bought: {}", amount_bought);

    let mut interval = tokio::time::interval(Duration::from_secs(9));
    loop {
        interval.tick().await;
        let last_price = get_last_price(client, symbol).await?;
        println!("Fetched last price: {}", last_price);

        if last_price > buy_price * 1.008 {
            println!("Pretend selling at price: {}", last_price);
            let sell_value = amount_bought * last_price;
            let profit = sell_value - 9.0; // Calculate profit based on amount bought with $9
            println!("Profit: {}", profit);
            break;
        }
    }

    Ok(())
}

async fn get_last_price(client: &Client, symbol: &str) -> Result<f64, reqwest::Error> {
    let url = format!(
        "https://api.gateio.ws/api/v4/spot/tickers?currency_pair={}",
        symbol
    );
    println!("Fetching last price from URL: {}", url);
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<Vec<serde_json::Value>>()
        .await?;
    let last_price = response[0]["last"]
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap();
    println!("Fetched last price: {}", last_price);
    Ok(last_price)
}

async fn monitor_pairs(client: Client) -> Result<(), reqwest::Error> {
    let symbols = [
        "BTC_USDT", "ETH_USDT", "LTC_USDT", "XRP_USDT", "TAO_USDT", "EOS_USDT", "TRX_USDT",
        "ADA_USDT", "XLM_USDT",
    ];
    println!("Starting to monitor: {:#?}", symbols);
    let mut interval = tokio::time::interval(Duration::from_secs(180));
    let start_time = Instant::now();
    loop {
        let mut got = false;
        for &symbol in &symbols {
            if got {
                continue;
            };
            if let Ok(ohlc) = get_ohlc(&client, symbol).await {
                if ohlc.len() >= 2 && check_trend(&ohlc) {
                    got = true;
                    println!("got {symbol}");
                    let buy_price = ohlc[1].close; // Record buy price
                    execute_trade(&client, symbol, buy_price).await?;
                }
            }
        }
        if got {
            if  start_time.elapsed().as_secs() < 27 {
                interval.tick().await;
            } else {
                println!("Time out")
            }
        }
    }
}

async fn _monitor_pair(client: Client, symbol: &str) -> Result<(), reqwest::Error> {
    println!("Starting to monitor pair: {}", symbol);
    let mut interval = tokio::time::interval(Duration::from_secs(180));
    loop {
        interval.tick().await;

        if let Ok(ohlc) = get_ohlc(&client, symbol).await {
            if ohlc.len() >= 2 && check_trend(&ohlc) {
                let buy_price = ohlc[1].close; // Record buy price
                sleep(Duration::from_secs(179)).await; // Wait 1 second before the candle ends (3 minutes - 1 second)
                execute_trade(&client, symbol, buy_price).await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Starting the application...");
    let client = Client::new();

    monitor_pairs(client).await.unwrap();

    /*
    let symbols = [
        "BTC_USDT", "ETH_USDT", "LTC_USDT", "XRP_USDT", "TAO_USDT", "EOS_USDT", "TRX_USDT",
        "ADA_USDT", "XLM_USDT",
    ];
    let mut handles = vec![];
    for &symbol in &symbols {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            //
            // dummy
            // let res = get_ohlc(&client, "TAO_USDT").await.unwrap();
            // println!("res: {:#?}", res);
            //
            monitor_pair(client_clone, symbol).await.unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    } */

    Ok(())
}
