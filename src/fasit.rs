use reqwest::{Client, Error};
use reqwest::header::LOCATION;
use reqwest::Response;
use serde_json;

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

pub(crate) fn create_resource<>(
    username: &str,
    password: &str,
    resource_name: &str,
    url: &str,
    env: &str,
    zone: &str) -> String {
    let request = json!({
        "type": "RestService",
          "alias": resource_name,
          "properties": {
            "url": url
          },
          "scope": {
            "environmentclass": get_env_class(env),
            "zone": zone,
            "environment": env,
          }
    });

    println!("Create RestService resource");
    println!("Posting: {}", serde_json::to_string(&request).unwrap());
    println!("To: {}", fasit_url());

    if let Ok(response) = Client::new()
        .post(&format!("{}/api/v2/resources", fasit_url()))
        .header("Content-Type", "application/json")
        .json(&request)
        .basic_auth(username, Some(password.to_owned()))
        .send() {
        if let Some(location) = response.headers().get(LOCATION) {
            location.to_str().unwrap().split("/").last().unwrap().to_owned()
        } else {
            "".to_owned()
        }
    } else {
        "".to_owned()
    }
}

pub(crate) fn create_application(
    username: &str,
    password: &str,
    application: &str,
    env: &str,
    resource_id: &str,
) {
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
        .basic_auth(username, Some(password.to_owned()))
        .send();
}
