use serde::{Deserialize, Serialize};
use super::cash_flow_frequency::CashFlowFrequency;

#[derive(Deserialize, Serialize)]
pub struct CashFlowDetail {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub value: f32,
    pub frequency: Option<CashFlowFrequency>,
}

impl CashFlowDetail {
    pub fn empty() -> Self {
        CashFlowDetail {
            id: None,
            name: String::new(),
            description: String::new(),
            value: 0.0,
            frequency: None,
        }
    }

    pub fn example() -> Self {
        CashFlowDetail {
            id: Some(String::from("123e4567-e89b-12d3-a456-426614174000")),
            name: String::from("Example cash flow"),
            description: String::from("You will be rich if this comes in a higher frequency with more value"),
            value: 42.24,
            frequency: Some(CashFlowFrequency::example())
        }
    }
}