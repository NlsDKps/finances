use serde::{Deserialize, Serialize};
use super::cash_flow_frequency::CashFlowFrequency;

#[derive(Deserialize, Serialize)]
pub struct CashFlowDetail {
    pub id: String,
    pub name: String,
    pub description: String,
    pub value: f32,
    pub frequency: Option<CashFlowFrequency>,
}

impl CashFlowDetail {
    pub fn empty() -> Self {
        CashFlowDetail {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            value: 0.0,
            frequency: None,
        }
    }
}