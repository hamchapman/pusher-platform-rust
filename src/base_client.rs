use crate::error::{Error};
use crate::instance::RequestOptions;
use serde::de::DeserializeOwned;
use reqwest;

pub struct BaseClient {
    host: String,
    port: u32,
    service_name: String,
    service_version: String,
    instance_id: String,
    // sdk_info: SDKInfo,
}

impl BaseClient {
    pub fn new() -> BaseClient {
        let http_client = reqwest::Client::new();
    }

    pub fn request<T: DeserializeOwned>(&self, options: RequestOptions) -> Result<T, Error> {

    }
}
