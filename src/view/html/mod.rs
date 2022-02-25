use rocket::response::Redirect;
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket_dyn_templates::Template;

use crate::view::structures::{CashFlow, cash_flow_detail::CashFlowDetail};

#[derive(Serialize)]
struct ContextReadAllFinances {
    cash_flows: Vec<CashFlow>
}

#[derive(Serialize)]
struct ContextReadCashFlowDetail {
    cash_flow: CashFlowDetail
}

#[derive(Serialize)]
struct ContextCreateCashFlow {

}

#[get("/cash_flow/create")]
pub fn create_cash_flow() -> Template {
    let context = ContextCreateCashFlow {};
    Template::render("create_cash_flow", &context)
}

#[put("/cash_flow/create", format="application/json", data="<cash_flow>")]
pub fn create_cash_flow_submit(cash_flow: Json<CashFlow>) -> Redirect {
    Redirect::to(uri!(read_cash_flow_all()))
}

#[get("/cash_flow/all")]
pub fn read_cash_flow_all() -> Template {
    let json_finances = r#"
    [{
        "name": "Lohn",
        "value": 5000
    }]
    "#;
    let cash_flows : Vec<CashFlow> = match serde_json::from_str(json_finances) {
        Ok(cf) => cf,
        Err(e) => { error!("Could not parse finances: {}", e); vec!() }
    };
    let context = ContextReadAllFinances {cash_flows};
    Template::render("read_cash_flow_all", &context)
}

#[get("/cash_flow/<id>")]
pub fn read_cash_flow(id: String) -> Template {
    let json_cash_flow = r#"
    {
        "id": "",
        "name": "Lohn",
        "value": 5000.00,
    }
    "#;
    let mut cash_flow : CashFlowDetail = match serde_json::from_str(json_cash_flow) {
        Ok(cf) => cf,
        Err(e) => { error!("Could not parse cash flow: {}", e); CashFlowDetail::empty() }
    };
    cash_flow.id = Some(id);
    let context = ContextReadCashFlowDetail {cash_flow};
    Template::render("read_cash_flow", &context)
}

#[get("/cash_flow/<id>/edit")]
pub fn update_cash_flow(id: String) -> Template {
    // Find element with specific id
    let json_cash_flow = r#"
    {
        "id": "",
        "name": "Lohn",
        "value": 5000.00,
        "frequency": "m"
    }
    "#;
    // Build and show element
    let mut cash_flow : CashFlowDetail = match serde_json::from_str(json_cash_flow) {
        Ok(cf) => cf,
        Err(e) => { error!("Could not parse cash flow: {}", e); CashFlowDetail::empty() }
    };
    cash_flow.id = Some(id);
    let context = ContextReadCashFlowDetail {cash_flow};
    Template::render("update_cash_flow", &context)
}

#[post("/cash_flow/details/<id>")]
pub fn update_cash_flow_submit(id: String) -> Redirect {
    Redirect::to(uri!(read_cash_flow_all()))
}

#[delete("/cash_flow/<id>")]
pub fn delete_cash_flow(id: String) -> Redirect {
    debug!("Deleting item with {}", id);
    Redirect::to(uri!(read_cash_flow_all))
}