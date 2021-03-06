use reqwest::{Client, Response, Error};
use serde_json;

use super::http_helpers::RestError;
use super::FasitUser;

#[cfg(not(test))]
fn api_management_url() -> &'static str { "http://api-management.default/rest" }

#[cfg(test)]
fn api_management_url() -> &'static str {
    "todo"
}

#[cfg(not(test))]
fn eier_group() -> &'static str { "Group_05b6c0d2-b6db-4440-96b4-4de66c09b3c6" }

#[cfg(test)]
fn eier_group() -> &'static str {
    "todo"
}

pub fn register_exposed_application(
    fasit_user: &FasitUser,
    application: &String,
    env: &str
) -> Result<Response, RestError> {
    info!("Register {} as exposed application in {}", application, env);

    Ok(http_ok_try!(Client::new()
        .put(&format!("{}/v1/katalog/applikasjoner/{}", api_management_url(), application))
        .query(&[("eier", eier_group()), ("sone", "TilbudtFraFss"), ("kilde", "fasit"), ("miljo", &env)])
        .basic_auth(&fasit_user.username, Some(fasit_user.password.clone()))
        .send()))
}

pub fn register_application_consumer(
    fasit_user: &FasitUser,
    application: &String,
    resource: &String,
    consumer: &String
) -> Result<Response, RestError> {
    info!("Register {} as consumer of {}", application, resource);

    Ok(http_ok_try!(Client::new()
        .put(&format!("{}/v1/katalog/applikasjoner/{}/{}/{}", api_management_url(), application, resource, consumer))
        .header("Content-Type", "application/json")
        .json(&json!({})) // Empty json means internal consumer
        .basic_auth(&fasit_user.username, Some(fasit_user.password.clone()))
        .send()))
}

pub fn register_application_consumer_connection(
    fasit_user: &FasitUser,
    application: &String,
    env: &str
) -> Result<Response, RestError> {
    info!("Register {} in rest gateway in {}", application, env);

    let request = json!({
        "gatewayEnv": env,
        "tilbyderEnv": env,
        "kommentar": &format!("Automatisk kobling av {}", application)
    });

    Ok(http_ok_try!(Client::new()
        .put(&format!("{}/v2/register/deploy/{}", api_management_url(), application))
        .header("Content-Type", "application/json")
        .json(&request)
        .basic_auth(&fasit_user.username, Some(fasit_user.password.clone()))
        .send()))
}
