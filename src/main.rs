use std::sync::mpsc;
use std::thread;

mod types;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let result = reqwest::blocking::get("https://api.coindesk.com/v1/bpi/currentprice.json");
        match result {
            Ok(response) => {
                tx.send(response.json::<types::Btc>()).unwrap();
            }
            Err(err) => {
                tx.send(Err(err)).unwrap();
            }
        };
    });

    for resp in rx.recv() {
        match resp {
            Ok(result) => println!("1 BTC = {:.0} €", result.bpi["EUR"].rate_float),
            Err(err) => println!("Error happened: {}", err),
        }
    }
}
