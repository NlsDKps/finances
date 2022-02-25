mod cash_flow_frequency;
pub(crate) mod cash_flow_detail;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CashFlow {
    pub name: String,
    pub value: f32
}

impl CashFlow {
    fn example() -> Self {
        CashFlow {
            name: String::from("Example Flow"),
            value: 123.4 as f32
        }
    }
}