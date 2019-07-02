use std::convert;

#[macro_use]
use log;
use reqwest::{Client, Error, StatusCode};
use reqwest::header::LOCATION;
use reqwest::Response;
use serde_json;

use super::FasitUser;
use super::http_helpers::RestError;

#[derive(Deserialize, Clone, Debug)]
pub struct FasitResource {
    pub id: u64,
    pub alias: String,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct FasitApplication {
    pub id: u64,
}

#[cfg(not(test))]
fn fasit_url() -> &'static str {
    "https://fasit.adeo.no"
}

#[cfg(test)]
fn fasit_url() -> &'static str {
    "todo"
}

fn get_env_class(env: &str) -> &str {
    match env {
        "p" => "p",
        _ => "q",
    }
}

impl From<reqwest::Error> for RestError {
    fn from(err: reqwest::Error) -> RestError {
        RestError::ReqwestError(err)
    }
}

pub fn get_fss_resource_by_name(resource_name: &String, env: &str) -> Result<Option<u64>, RestError> {
    info!("Atempting to get resource with name: {}, in {}", resource_name, env);

    Ok(http_ok_try!(Client::new()
        .get(&format!("{}/api/v2/resources", fasit_url()))
        .query(&[
            ("alias", resource_name),
            ("environment", &env.to_owned()),
            ("zone", &"FSS".to_owned()),
            ("type", &"RestService".to_owned())
        ])
        .send())
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
    env: &str) -> Result<u64, RestError> {
    info!("Creating resource: {} in {}", resource_name, env);

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

    Ok(http_ok_try!(Client::new()
        .post(&format!("{}/api/v2/resources", fasit_url()))
        .header("Content-Type", "application/json")
        .json(&request)
        .basic_auth(&fasit_user.username, Some(fasit_user.password.clone()))
        .send())
        .headers().get(LOCATION).unwrap()
        .to_str().unwrap().split("/").last().unwrap().to_owned().parse::<u64>().unwrap())
}

pub fn create_application(fasit_user: &FasitUser, application: &str) -> Result<u64, RestError> {
    info!("Creating application: {}", application);

    let request = json!({
        "name": application,
        "groupId": "no.nav.syfo",
        "artifactId": application,
        "portOffset": 0
    });

    Ok(http_ok_try!(Client::new()
        .post(&format!("{}/api/v2/applications", fasit_url()))
        .header("Content-Type", "application/json")
        .json(&request)
        .basic_auth(&fasit_user.username, Some(fasit_user.password.clone()))
        .send())
        .headers().get(LOCATION).unwrap()
        .to_str().unwrap().split("/").last().unwrap().to_owned().parse::<u64>().unwrap())
}

pub fn get_application_by_name(application: &str) -> Result<u64, RestError> {
    info!("Atempting to get application: {}", application);

    Ok(http_ok_try!(Client::new()
        .get(&format!("{}/api/v2/applications/{}", fasit_url(), application))
        .send())
        .json::<FasitApplication>().unwrap().id)
}

pub fn create_application_instance(
    fasit_user: &FasitUser,
    application: &str,
    env: &str,
    resource_ids: Vec<u64>,
) -> Result<reqwest::Response, RestError> {
    info!("Creating application instance for application: {} with exposed resource: {:?} in {}",
          application, resource_ids, env);

    let exposed_resources: Vec<_> = resource_ids.into_iter().map(|id| json!({ "id": id })).collect();

    let request = json!({
        "application": application,
        "version": "1.0.0",
        "environment": env,
        "clustername": "nais",
        "exposedresources": exposed_resources
    });

    Ok(http_ok_try!(Client::new()
        .post(&format!("{}/api/v2/applicationinstances", fasit_url()))
        .header("Content-Type", "application/json")
        .json(&request)
        .basic_auth(&fasit_user.username, Some(fasit_user.password.clone()))
        .send()))
}
