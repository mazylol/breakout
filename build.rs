use std::env;
use copy_to_output::copy_to_output;

fn main() {
    println!("cargo:rerun-if-changed=res/*");
    copy_to_output("res", &env::var("PROFILE").unwrap()).expect("Could not copy");
}