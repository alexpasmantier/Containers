use std::io;
use std::path::Path;

use tar::Archive;

use bytes::Bytes;
use flate2::bufread::MultiGzDecoder;
use reqwest::{blocking::Client, Error};

use crate::parsing::arguments::Image;

use super::{
    authentication::Auth,
    manifest::{ImageManifest, ImageManifestLayer},
    DOCKERHUB_REGISTRY_BASE_URL,
};

pub fn pull_layers(
    image: &Image,
    manifest: &ImageManifest,
    auth: &Auth,
) -> Result<Vec<Bytes>, Error> {
    manifest
        .layers
        .iter()
        .map(|layer_manifest| get_layer(image, layer_manifest, auth))
        .collect()
}

pub fn get_layer<'a>(
    image: &Image,
    manifest: &'a ImageManifestLayer,
    auth: &Auth,
) -> Result<Bytes, Error> {
    let layer_url = format!(
        "{base_url}/v2/library/{name}/blobs/{digest}",
        base_url = DOCKERHUB_REGISTRY_BASE_URL,
        name = image.repository,
        digest = manifest.digest
    );
    // println!("getting layer @ {}", &layer_url);
    match Client::new()
        .get(layer_url)
        .bearer_auth(&auth.token)
        .send()?
        .error_for_status()
    {
        Ok(response) => {
            // println!("done");
            return Ok(response.bytes().unwrap());
        }
        Err(e) => return Err(e),
    }
}

pub fn decompress(content: &Bytes, destination_path: &Path) -> io::Result<()> {
    let decoder = MultiGzDecoder::new(content.as_ref());
    let mut archive = Archive::new(decoder);
    archive.unpack(destination_path)
}
