use clap::{crate_version, App};

fn main() {
    App::new("F1 API").version(crate_version!()).get_matches();
}
