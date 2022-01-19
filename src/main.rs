use std::process;

fn main() {
    if let Err(e) = crate_r::run() {
        println!("oops: {}", e);
        process::exit(1);
    }
}
