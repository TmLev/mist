use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
pub struct Vm {
    // Number of virtual CPU cores
    pub cpu: u32,

    // Megabytes of RAM
    pub mem: u32,
}

pub type Price = f64;
pub type BillingInterval = f64;
pub type Cost = f64; // price * billing_period

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct InstanceType {
    // VM characteristics
    pub vm: Vm,

    // Price per `billing_period` in USD
    pub price: Price,

    // Interval of time between billings in seconds
    pub billing_interval: BillingInterval,
}

impl InstanceType {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self)
    }

    pub fn from_json(raw: &str) -> serde_json::Result<Self> {
        serde_json::from_str(raw)
    }
}
