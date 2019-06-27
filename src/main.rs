extern crate env_logger;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod fasit;
mod api_management;

#[derive(Deserialize, Clone, Debug)]
pub struct FasitUser {
    username: String,
    password: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationConsumerPair {
    application_name: String,
    consumer_name: String,
}

enum Zone {
    FSS,
    SBS,
}

impl Zone {
    fn application_url_for(&self, application_name: &String, env: &str) -> String {
        format!("https://{}.{}", application_name, self.domain_name_for(env))
    }

    fn domain_name_for(&self, env: &str) -> &'static str {
        match self {
            Zone::FSS => match env {
                "p" => "nais.adeo.no",
                _ => "nais.preprod.local",
            },
            Zone::SBS => match env {
                "p" => "nais.oera.no",
                _ => "nais.oera-q.no",
            }
        }
    }
}

#[cfg(not(test))]
fn fasit_user() -> FasitUser { FasitUser { username: "usr".to_owned(), password: "pass".to_owned() } }

fn main() {
    env_logger::init();

    let credentials_path: PathBuf = [env::var("VAULT_PATH")
        .unwrap_or(".".to_owned()), "credentials.json".to_owned()].iter().collect();
    info!("Secrets path: {:?}", &credentials_path);

    let fasit_user: FasitUser = serde_json::from_reader(
        File::open(&credentials_path).expect("Unable to open secrets file")
    ).expect("Unable to parse secrets as json");

    let configuration: Vec<ApplicationConsumerPair> = serde_json::from_reader(
        File::open("configuration.json").expect("Unable to open configuration file")
    ).expect("Unable to parse configuration as json");

    (&configuration).into_iter()
        .flat_map(|pair| vec![("p", pair), ("q1", pair)])
        .for_each(|(env, pair)| {
            connect_consumer_and_producer(
                &fasit_user,
                &pair.application_name,
                &pair.consumer_name,
                env,
            )
        });

    info!("{:?}", &configuration);
}

fn connect_consumer_and_producer(
    fasit_user: &FasitUser,
    application_name: &String,
    consumer_name: &String,
    env: &str) {
    let application_resource_name =
        set_up_applicationinstance(fasit_user, application_name, &Zone::FSS, env);

    set_up_applicationinstance(fasit_user, consumer_name, &Zone::SBS, env);

    api_management::register_exposed_application(
        fasit_user,
        application_name,
        env,
    )
        .map(|mut result|
            info!("Register resource call with status: {}, and body: {:?}",
                  result.status(),
                  result.json::<serde_json::Value>()))
        .expect("Failed to register exposed application");

    api_management::register_application_consumer(
        fasit_user,
        application_name,
        &application_resource_name,
        consumer_name,
    )
        .map(|mut result|
            info!("Register application consumer call with status: {}, and body: {:?}",
                  result.status(),
                  result.json::<serde_json::Value>()))
        .expect("Failed to register application consumer");

    api_management::register_application_consumer_connection(
        fasit_user,
        application_name,
        env,
    )
        .map(|mut result|
            info!("Register application consume connection call with status: {}, and body: {:?}",
                  result.status(),
                  result.json::<serde_json::Value>()))
        .expect("Failed to register application consumer connection");

    info!("Application: {} and consumer: {} connected in {}", application_name, consumer_name, env)
}

fn set_up_applicationinstance(
    fasit_user: &FasitUser,
    application_name: &String,
    zone: &Zone,
    env: &str) -> String {
    let resource_name = format!("{}Api", application_name);
    let url = zone.application_url_for(application_name, env);

    let resource_id = get_or_create_resource(&fasit_user, &resource_name, &url, env);
    get_or_create_application(&fasit_user, application_name);

    fasit::create_application_instance(fasit_user, application_name, env, &resource_id)
        .expect("Failed to create application instance");

    resource_name
}

fn get_or_create_resource(
    fasit_user: &FasitUser,
    resource_name: &String,
    url: &String,
    env: &str) -> u64 {
    fasit::get_resource_by_name(resource_name, env)
        .expect("Failed to get resource from fasit")
        .unwrap_or_else(|| fasit::create_resource(fasit_user, resource_name, url, env)
            .expect("Failed to create resource in fasit"))
}

fn get_or_create_application(fasit_user: &FasitUser, appliaction: &String) -> u64 {
    fasit::get_application_by_name(appliaction)
        .expect("Failed to get application from fasit")
        .unwrap_or_else(|| fasit::create_application(fasit_user, appliaction)
            .expect("Failed to create application in fasit"))
}
