extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod fasit;
mod api_management;

fn main() {
    fasit::create_resource("usr", "pass", "applicationApi", "url", "q1", "FSS");
    fasit::create_application("usr", "pass", "application", "q1", "818281");
    api_management::register_exposed_application("usr", "pass", "application");
    api_management::register_application_consumer("usr", "pass", "application", "service", "consumer");
    api_management::register_application_consumer_connection("usr", "pass", "application");
}
