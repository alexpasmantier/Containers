use reqwest::{blocking::Client, header::ACCEPT, Error};
use serde::Deserialize;

use crate::parsing::arguments::Image;

use super::{authentication::Auth, DOCKERHUB_REGISTRY_BASE_URL};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageManifestConfig {
    pub media_type: String,
    pub size: u32,
    pub digest: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageManifestLayer {
    pub media_type: String,
    pub size: u32,
    pub digest: String,
    pub urls: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageManifest {
    pub schema_version: u8,
    pub media_type: String,
    pub config: ImageManifestConfig,
    pub layers: Vec<ImageManifestLayer>,
}

const MANIFEST_MEDIA_TYPE: &str = "application/vnd.docker.distribution.manifest.v2+json";
const DEFAULT_TAG: &str = "latest";

// https://registry.hub.docker.com/v2/library/repository/manifests/tag
pub fn get_manifest(target_image: &Image, auth: &Auth) -> Result<ImageManifest, Error> {
    // this is kind of ugly and should probably use a proper url builder instead
    let url = format!(
        "{base_url}/v2/library/{repository}/manifests/{tag}",
        base_url = DOCKERHUB_REGISTRY_BASE_URL,
        repository = target_image.repository,
        tag = target_image.tag.clone().unwrap_or(DEFAULT_TAG.to_string())
    );
    // println!("getting manifest @ {}", &url);
    match Client::new()
        .get(url)
        .bearer_auth(&auth.token)
        .header(ACCEPT, MANIFEST_MEDIA_TYPE)
        .send()
    {
        Ok(response) => {
            // println!("done");
            return Ok(response
                .json::<ImageManifest>()
                .expect("Unable to deserialize image manifest"));
        }
        Err(e) => return Err(e),
    }
}
