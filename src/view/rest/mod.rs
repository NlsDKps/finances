use rocket::serde::{json::Json};
use crate::view::structures::{CashFlow, cash_flow_detail::CashFlowDetail};

pub fn create_cash_flow() -> Json<CashFlowDetail> {
    Json(CashFlowDetail::empty())
}

pub fn read_cash_flow_all() -> Json<Vec<CashFlow>> {
    Json(vec!())
}

pub fn read_cash_flow(_id: i32) -> Json<CashFlowDetail> {
    Json(CashFlowDetail::empty())
}

pub fn update_cash_flow(_cash_flow: CashFlowDetail) -> Json<CashFlowDetail> {
    Json(CashFlowDetail::empty())
}

pub fn delete_cash_flow(_id: i32) -> Json<CashFlowDetail> {
    Json(CashFlowDetail::empty())
}