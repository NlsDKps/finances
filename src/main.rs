#[macro_use]extern crate rocket;

use finances::view::html;
use finances::view::api;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    let html_routes = routes![
        html::create_cash_flow,
        html::create_cash_flow_submit,
        html::read_cash_flow,
        html::read_cash_flow_all,
        html::update_cash_flow,
        html::update_cash_flow_submit,
        html::delete_cash_flow,
    ];
    let api_routes = routes![
        api::create_cash_flow,
        api::read_cash_flow_all,
        api::read_cash_flow,
        api::update_cash_flow,
        api::delete_cash_flow
    ];
    rocket::build().mount("/", html_routes).mount("/api/", api_routes).attach(Template::fairing())
}
