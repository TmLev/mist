use serde::{Deserialize, Serialize};

/// Reflects virtual machine (VM) that runs applications or services.
#[derive(Debug, Clone)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
pub struct VirtualMachine {
    /// Number of virtual CPU cores.
    pub cpu: u32,

    /// Megabytes of RAM.
    pub mem: u32,
}

pub type Price = f64;
pub type BillingInterval = f64;
pub type Cost = f64; // price * billing_interval

/// Virtual machine (VM) available for renting in the public cloud.
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct InstanceType {
    /// Actual characteristics.
    pub characteristics: VirtualMachine,

    /// Price per `billing_period` in USD.
    pub price: Price,

    /// Interval of time between billings in seconds.
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
