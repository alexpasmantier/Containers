use reqwest::blocking::Client;
use reqwest::Error;
use serde::Deserialize;

use crate::parsing::arguments::Image;

const AUTHENTICATION_URL: &str = "https://auth.docker.io/token";
const SERVICE: &str = "registry.docker.io";

#[derive(Deserialize, Debug)]
pub struct Auth {
    pub token: String,
}

pub fn authenticate(target_image: &Image) -> Result<Auth, Error> {
    let scope = format!("repository:library/{}:pull", target_image.repository);
    // println!("authenticating");
    match Client::new()
        .get(AUTHENTICATION_URL)
        .query(&[("service", SERVICE), ("scope", &scope)])
        .send()
    {
        Ok(response) => {
            // println!("done");
            return Ok(response
                .json::<Auth>()
                .expect("Unable to deserialize response"));
        }
        Err(e) => {
            eprintln!("Error getting authentication token: {}", e);
            return Err(e);
        }
    }
}
