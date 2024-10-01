use findctl::*;
use std::env;
use std::process;
use termcolor::Color;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    write_color("query", &config.query, Color::Green);
    if let Err(e) = run(config) {
        eprintln!("run error: {}", e);
        process::exit(1)
    }
}
