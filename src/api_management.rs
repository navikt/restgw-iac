use reqwest::{Client, Error};
use serde_json;

#[cfg(not(test))]
fn api_management_url() -> &'static str { "http://localhost:3000" }
#[cfg(not(test))]
fn eier_group() -> &'static str { "Group_05b6c0d2-b6db-4440-96b4-4de66c09b3c6" }

pub(crate) fn register_exposed_application(
    username: &str,
    password: &str,
    applikasjon: &str
) {
    println!("Register application in api-management");

    Client::new()
        .put(&format!("{}/v1/katalog/applikasjoner/{}", api_management_url(), applikasjon))
        .query(&[("eier", eier_group()), ("sone", "TilbudtFraFss"), ("kilde", "fasit"), ("miljo", "q1")])
        .basic_auth(username, Some(password.to_owned()))
        .send();
}

pub(crate) fn register_application_consumer(
    username: &str,
    password: &str,
    applikasjon: &str,
    tjeneste: &str,
    konsument: &str
) {
    println!("Register application consumer access in api-management");

    Client::new()
        .put(&format!("{}/v1/katalog/applikasjoner/{}/{}/{}", api_management_url(), applikasjon, tjeneste, konsument))
        .header("Content-Type", "application/json")
        .json(&json!({})) // Empty json means internal consumer
        .basic_auth(username, Some(password.to_owned()))
        .send();
}

pub(crate) fn register_application_consumer_connection(
    username: &str,
    password: &str,
    applikasjon: &str,
) {
    println!("Register api-gw connection between application and consumer");

    let request = json!({
        "gatewayEnv": "q1",
        "tilbyderEnv": "q1",
        "kommentar": &format!("Automatisk kobling av {}", applikasjon)
    });

    Client::new()
        .post(&format!("{}/v2/deploy/{}", api_management_url(), applikasjon))
        .header("Content-Type", "application/json")
        .json(&request)
        .basic_auth(username, Some(password.to_owned()))
        .send();
}
