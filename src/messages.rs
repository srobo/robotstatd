use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryInfo {
    pub used: u64,
    pub usercode: u64,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatsMessage {
    pub cpu_usage: Vec<f32>,
    pub memory: MemoryInfo,
}
