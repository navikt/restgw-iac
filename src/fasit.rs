use std::convert;

use reqwest::{Client, Error, StatusCode};
use reqwest::header::LOCATION;
use reqwest::Response;
use serde_json;

use crate::FasitUser;

#[cfg(not(test))]
fn fasit_url() -> &'static str {
    "http://localhost:3000"
}

fn get_env_class(env: &str) -> &str {
    return if env.contains("p") {
        "p"
    } else {
        "q"
    };
}

pub fn create_resource(
    fasit_user: &FasitUser,
    resource_name: &String,
    url: &String,
    env: &str) -> reqwest::Result<u64> {
    let request = json!({
        "type": "RestService",
          "alias": resource_name,
          "properties": {
            "url": url
          },
          "scope": {
            "environmentclass": get_env_class(env),
            "zone": "FSS",
            "environment": env,
          }
    });

    println!("Create RestService resource");
    println!("Posting: {}", serde_json::to_string(&request).unwrap());
    println!("To: {}", fasit_url());

    Ok(Client::new()
        .post(&format!("{}/api/v2/resources", fasit_url()))
        .header("Content-Type", "application/json")
        .json(&request)
        .basic_auth(&fasit_user.username, Some(fasit_user.password.clone()))
        .send()?
        .headers().get(LOCATION).unwrap()
        .to_str().unwrap().split("/").last().unwrap().to_owned().parse::<u64>().unwrap())
}

pub fn create_application(fasit_user: &FasitUser, application: &str) -> reqwest::Result<u64> {
    let request = json!({
        "name": application,
        "groupId": "no.nav.syfo",
        "artifactId": application,
        "portOffset": 0
    });

    Ok(Client::new()
        .post(&format!("{}/api/v2/applications", fasit_url()))
        .header("Content-Type", "application/json")
        .json(&request)
        .basic_auth(&fasit_user.username, Some(fasit_user.password.clone()))
        .send()?
        .headers().get(LOCATION).unwrap()
        .to_str().unwrap().split("/").last().unwrap().to_owned().parse::<u64>().unwrap())
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct FasitResource {
    pub id: u64,
}

pub fn get_application_by_name(application: &str) -> reqwest::Result<Option<FasitResource>> {
    Client::new()
        .get(&format!("{}/api/v2/applications/{}", fasit_url(), application))
        .send()
        .map(|mut response| Ok(Some(response.json::<FasitResource>().unwrap())))
        .unwrap_or_else(|err| match err.status().map(|status| status.as_u16()) {
            Some(404) => Ok(None),
            _ => Err(err),
        })
}

pub fn create_application_instance(
    fasit_user: &FasitUser,
    application: &str,
    env: &str,
    resource_id: &u64,
) -> reqwest::Result<reqwest::Response> {
    let request = json!({
        "application": application,
        "version": "1.0.0",
        "environment": env,
        "clustername": "nais",
        "exposedresources": [{
            "id": resource_id
        }]
    });

    println!("Create application");
    println!("Posting: {}", serde_json::to_string(&request).unwrap());
    println!("To: {}", fasit_url());

    Client::new()
        .post(&format!("{}/api/v2/applicationinstances", fasit_url()))
        .header("Content-Type", "application/json")
        .json(&request)
        .basic_auth(&fasit_user.username, Some(fasit_user.password.clone()))
        .send()
}
