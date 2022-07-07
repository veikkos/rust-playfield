use std::error::Error;

mod types;

async fn get() -> Result<types::Btc, Box<dyn Error>> {
    Ok(
        reqwest::get("https://api.coindesk.com/v1/bpi/currentprice.json")
            .await?
            .json::<types::Btc>()
            .await?,
    )
}

#[tokio::main]
async fn main() {
    let resp = get().await;
    match resp {
        Ok(result) => println!("1 BTC = {:.0} €", result.bpi["EUR"].rate_float),
        Err(err) => println!("Error happened: {}", err),
    }
}
