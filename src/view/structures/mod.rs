mod cash_flow_frequency;
pub(crate) mod cash_flow_detail;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CashFlow {
    pub name: String,
    pub value: f32
}
