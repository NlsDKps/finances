#[macro_use]extern crate rocket;

use finances::view::html::{
    create_cash_flow, create_cash_flow_submit,
    read_cash_flow_all, read_cash_flow,
    update_cash_flow, update_cash_flow_submit,
    delete_cash_flow
};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    let cash_flow_routes = routes![
        create_cash_flow,
        create_cash_flow_submit,
        read_cash_flow,
        read_cash_flow_all,
        update_cash_flow,
        update_cash_flow_submit,
        delete_cash_flow
    ];
    rocket::build().mount("/", cash_flow_routes).attach(Template::fairing())
}
