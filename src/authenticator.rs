use std::time::{Duration, SystemTime, UNIX_EPOCH};

use reqwest;
use jsonwebtoken::{encode, Header, Algorithm};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    instance: String,
    iss: String,
    exp: u64,
    iat: u64,
    sub: String,
}


const DEFAULT_TOKEN_EXPIRY: u64 = 24*60*60;

pub struct Authenticator {
    instance_id: String,
    key_id: String,
    key_secret: String,
    /// The underlying Hyper HTTP client.
    http_client: reqwest::Client,
}


impl Authenticator {
    pub fn new(instance_id: &str, key_id: &str, key_secret: &str) -> Authenticator {
        let http_client = reqwest::Client::new();

        Authenticator {
          instance_id: instance_id.to_string(),
          key_id: key_id.to_string(),
          key_secret: key_secret.to_string(),
          http_client: http_client,
        }
    }

    pub fn generate_access_token(&self, sub: &str) -> TokenWithExpiry {
        let start = SystemTime::now();
        let now = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

        // let now = Math.floor(Date.now() / 1000);
        // let tokenExpiry = options.tokenExpiry || this.tokenExpiry;

        let token_expiry = Duration::from_secs(DEFAULT_TOKEN_EXPIRY);

        let claims = Claims {
          instance: self.instance_id.to_string(),
          iss: format!("api_keys/{}", self.key_id),
          exp: now.checked_add(token_expiry).expect("Time went wrong").as_secs(),
          iat: now.as_secs(),
          sub: sub.to_string(),
        };

        let key = self.key_secret.to_string();

        let mut header = Header::default();
        header.alg = Algorithm::HS256;

        // let token = match encode(&header, &claims, key.as_ref()) {
        //     Ok(t) => t,
        //     Err(_) => panic!(), // in practice you would return the error
        // };
        // println!("{:?}", token);

        let token = encode(&header, &claims, key.as_ref()).expect("JWT signing should work");

        TokenWithExpiry {
          token: token,
          expires_in: token_expiry.as_secs(),
        }
    }
}

pub struct TokenWithExpiry {
    token: String,
    expires_in: u64,
}
