extern crate pusher_platform_rust;

use pusher_platform_rust::{Instance};

fn main() {
    let instance = Instance::new(
        "v1:us1:233b0361-4b6e-4095-bc3c-82fb9081e406",
        "d52e23cc-5b56-4428-a64e-f739eb870923:pAfiVTS1edLEfwI8shyt1hpCgJwC2cuMvX42L9QeRHg=",
        "chatkit",
        "v2",
    );

    print!("{}", instance.generate_access_token("ham").token);
}
