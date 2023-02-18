use std::{env, process};

use minigrep_tay::Config;


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) =  minigrep_tay::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}
