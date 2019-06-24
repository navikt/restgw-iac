extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod fasit;
mod api_management;

pub struct FasitUser {
    username: String,
    password: String,
}

#[cfg(not(test))]
fn fasit_user() -> FasitUser { FasitUser { username: "usr".to_owned(), password: "pass".to_owned() } }

fn applikasjonsnavn() -> &'static str { "application" }

fn main() {

}

fn set_up_and_connect_consumer_and_producer(fasit_user: &FasitUser, application_name: &String, consumer_name: &String, env: &str) {
    // Setter opp konsumert tjeneste fra fss
    let application_tjenestenavn = format!("{}Api", application_name);
    let tjeneste_url = if env == "p" {
        format!("https://{}.nais.adeo.no", application_name)
    } else {
        format!("https://{}.nais.preprod.local", application_name)
    };
    let producer_ressurs_id = fasit::create_resource(fasit_user, &application_tjenestenavn, &tjeneste_url, env)
        .expect("Failed to find or create resource");
    let application_resource = fasit::get_application_by_name(application_name)
        .expect("Failed to get resource")
        .and_then(|resource| Some(resource.id))
        .unwrap_or_else(|| fasit::create_application(fasit_user, application_name)
            .expect("Failed to create application"));
    fasit::create_application_instance(fasit_user, application_name, env, &producer_ressurs_id);

    // Setter opp konsument i SBS
    let consumer_tjenestenavn = format!("{}Api", consumer_name);
    let konsument_url = if env == "p" {
        format!("https://{}.nais.oera.no", application_name)
    } else {
        format!("https://{}.nais.oera-q.local", application_name)
    };
    let consumer_ressurs_id = fasit::create_resource(fasit_user, &consumer_tjenestenavn, &konsument_url, env)
        .expect("Failed to find or create resource");
    let consumer_application_resource = fasit::get_application_by_name(consumer_name)
        .expect("Failed to get resource")
        .and_then(|resource| Some(resource.id))
        .unwrap_or_else(|| fasit::create_application(fasit_user, consumer_name)
            .expect("Failed to create application"));
    fasit::create_application_instance(fasit_user, consumer_name, env, &consumer_ressurs_id);


    // Api management shit
    api_management::register_exposed_application(fasit_user, application_name);
    api_management::register_application_consumer(fasit_user, application_name, &application_tjenestenavn, consumer_name);
    api_management::register_application_consumer_connection(fasit_user, application_name, env);
}
