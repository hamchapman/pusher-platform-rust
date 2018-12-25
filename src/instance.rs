use crate::base_client::BaseClient;
use crate::authenticator::{Authenticator, TokenWithExpiry};

const HOST_BASE: &str = "pusherplatform.io";
const HTTPS_PORT: u32 = 443;

pub struct Instance {
  client: BaseClient,
  id: String,
  key_id: String,
  key_secret: String,

  service_name: String,
  service_version: String,

  host: String,

  cluster: String,
  platform_version: String,

  authenticator: Authenticator,
}

impl Instance {
    pub fn new(locator: &str, key: &str, service_name: &str, service_version: &str) -> Instance {
        let split_locator: Vec<&str> = locator.split(':').collect();
        let platform_version = split_locator[0];
        let cluster = split_locator[1];
        let instance_id = split_locator[2];

        let split_key: Vec<&str> = key.split(':').collect();
        let key_id = split_key[0];
        let key_secret = split_key[1];

        let authenticator = Authenticator::new(instance_id, key_id, key_secret);

        let base_client = BaseClient{};

        Instance {
          id: instance_id.to_string(),
          key_id: key_id.to_string(),
          key_secret: key_secret.to_string(),
          client: base_client,
          authenticator: authenticator,
          cluster: cluster.to_string(),
          platform_version: platform_version.to_string(),
          service_version: service_version.to_string(),
          service_name: service_name.to_string(),
          host: HOST_BASE.to_string(),
        }
    }

    pub fn generateAccessToken(&self, sub: &str) -> TokenWithExpiry {
        return self.authenticator.generate_access_token(sub);
    }
}
