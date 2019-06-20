use reqwest::{Client, Error};

#[cfg(not(test))]
fn fasit_url() -> &'static str {
    "http://fasit"
}

fn create_resource(username: &str, password: &str, alias: &str, env: &str, zone: &str) {

    let env_class = if env.contains("p") {
        "p"
    } else {
        "q"
    };

    let request = json!({
        "type": "RestService",
          "alias": alias,
          "scope": {
            "environmentclass": env_class,
            "zone": zone,
            "environment": env,
            "application": "All applications"
          }
    });

    Client::new()
        .post(&format!("{}", fasit_url()))
        .json(&request)
        .basic_auth(username, Some(password.to_owned()));
}
