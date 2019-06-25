use std::convert;

use reqwest::{Client, Error, StatusCode};
use reqwest::header::LOCATION;
use reqwest::Response;
use serde_json;

use crate::FasitUser;

#[derive(Deserialize, Clone, Debug)]
pub struct FasitResource {
    pub id: u64,
    pub alias: String,
}

#[cfg(not(test))]
fn fasit_url() -> &'static str {
    "http://localhost:3000"
}

#[cfg(test)]
fn fasit_url() -> &'static str {
    "todo"
}

fn get_env_class(env: &str) -> &str {
    match env {
        p => "p",
        _ => "q",
    }
}

pub fn get_resource_by_name(resource_name: &String, env: &str) -> reqwest::Result<Option<u64>> {
    Ok(Client::new()
        .get(&format!("{}/api/v2/resources", fasit_url()))
        .query(&[("alias", resource_name), ("environment", &env.to_owned())])
        .send()?
        .json::<Vec<FasitResource>>()?
        .into_iter()
        .find(|resource| &resource.alias == resource_name)
        .map(|resource| resource.id)
    )
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

pub fn get_application_by_name(application: &str) -> reqwest::Result<Option<u64>> {
    Client::new()
        .get(&format!("{}/api/v2/applications/{}", fasit_url(), application))
        .send()
        .map(|mut response| Ok(Some(response.json::<FasitResource>().unwrap().id)))
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
