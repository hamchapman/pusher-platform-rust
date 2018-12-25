#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod instance;
mod base_client;
mod authenticator;

pub use self::instance::{Instance};
