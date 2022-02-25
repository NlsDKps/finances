use rocket::serde::{json::Json};
use crate::view::structures::{CashFlow, cash_flow_detail::CashFlowDetail};

#[post("/cash_flow", format="application/json", data="<cash_flow>")]
pub fn create_cash_flow(cash_flow: Json<CashFlowDetail>) -> Json<CashFlowDetail> {
    cash_flow
}

#[get("/cash_flow")]
pub fn read_cash_flow_all() -> Json<Vec<CashFlow>> {
    Json(vec!())
}

#[get("/cash_flow/<id>")]
pub fn read_cash_flow(id: String) -> Json<CashFlowDetail> {
    let mut cash_flow = CashFlowDetail::example();
    cash_flow.id = Some(id);
    Json(cash_flow)
}

#[put("/cash_flow/<id>", format="application/json", data="<json_cash_flow>")]
pub fn update_cash_flow(id: String, mut json_cash_flow: Json<CashFlowDetail>) -> Json<CashFlowDetail> {
    json_cash_flow.id = Some(id);
    json_cash_flow
}

#[delete("/cash_flow/<id>")]
pub fn delete_cash_flow(id: String) -> Json<CashFlowDetail> {
    let mut cash_flow = CashFlowDetail::example();
    cash_flow.id = Some(id);
    Json(cash_flow)
}