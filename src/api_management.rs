use reqwest::{Client, Error};
use serde_json;

use crate::FasitUser;

#[cfg(not(test))]
fn api_management_url() -> &'static str { "http://localhost:3000" }
#[cfg(not(test))]
fn eier_group() -> &'static str { "Group_05b6c0d2-b6db-4440-96b4-4de66c09b3c6" }

pub fn register_exposed_application(
    fasit_user: &FasitUser,
    application: &String,
    env: &str
) -> reqwest::Result<reqwest::Response> {
    Client::new()
        .put(&format!("{}/v1/katalog/applikasjoner/{}", api_management_url(), application))
        .query(&[("eier", eier_group()), ("sone", "TilbudtFraFss"), ("kilde", "fasit"), ("miljo", &env)])
        .basic_auth(&fasit_user.username, Some(fasit_user.password.clone()))
        .send()
}

pub fn register_application_consumer(
    fasit_user: &FasitUser,
    application: &String,
    resource: &String,
    consumer: &String
) -> reqwest::Result<reqwest::Response> {
    Client::new()
        .put(&format!("{}/v1/katalog/applikasjoner/{}/{}/{}", api_management_url(), application, resource, consumer))
        .header("Content-Type", "application/json")
        .json(&json!({})) // Empty json means internal consumer
        .basic_auth(&fasit_user.username, Some(fasit_user.password.clone()))
        .send()
}

pub fn register_application_consumer_connection(
    fasit_user: &FasitUser,
    application: &String,
    env: &str
) -> reqwest::Result<reqwest::Response> {
    let request = json!({
        "gatewayEnv": env,
        "tilbyderEnv": env,
        "kommentar": &format!("Automatisk kobling av {}", application)
    });

    Client::new()
        .post(&format!("{}/v2/deploy/{}", api_management_url(), application))
        .header("Content-Type", "application/json")
        .json(&request)
        .basic_auth(&fasit_user.username, Some(fasit_user.password.clone()))
        .send()
}