mod aux;

use std::env;
use std::process::exit;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        println!("Error. Incorrect number of argumenta supplied");
        println!("Usage: {} <ip addr>", argv[0]);
        exit(1);
    }

    let output = aux::ping(&argv[1]);
    let ttl = aux::get_ttl(&output);

    aux::check_os(&ttl, &argv[1]);

}
