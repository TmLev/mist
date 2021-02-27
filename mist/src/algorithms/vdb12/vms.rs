use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Vm {
    // Number of virtual CPU cores
    pub cpu: u32,

    // Megabytes of RAM
    pub mem: u32,
}

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct InstanceType {
    // VM characteristics
    pub vm: Vm,

    // Price in USD
    pub price: f64,

    // Interval of time between billings in seconds
    pub billing_period: f64,
}

impl InstanceType {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self)
    }

    pub fn from_json(raw: &str) -> serde_json::Result<Self> {
        serde_json::from_str(raw)
    }
}
