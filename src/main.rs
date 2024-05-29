use std::env;
use std::process;
use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    eprintln!("{:?}", args); // read out eprintln once.

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Issue while parsing argument -> {}", err);
        process::exit(01);
    });

    if let Err(e) = grep::run(config){
        eprintln!("Application error! {}", e);
        process::exit(1);
    }
}
