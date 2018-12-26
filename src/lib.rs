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
extern crate erased_serde;

mod authenticator;
mod base_client;
mod error;
mod instance;

pub use self::instance::{Instance};
