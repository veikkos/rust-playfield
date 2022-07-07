#![allow(non_snake_case)]
#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Time {
    pub updated: String,
    pub updatedISO: String,
    pub updateduk: String,
}

#[derive(Deserialize)]
pub struct Currency {
    pub code: String,
    pub symbol: String,
    pub rate: String,
    pub description: String,
    pub rate_float: f64,
}

#[derive(Deserialize)]
pub struct Btc {
    pub time: Time,
    pub disclaimer: String,
    pub chartName: String,
    pub bpi: HashMap<String, Currency>,
}
