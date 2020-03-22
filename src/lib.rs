#[macro_use]
extern crate log;
extern crate simple_logger;

mod config;
mod errors;
mod files;
mod network;
mod types;

use types::Result;

const FLAG_VERBOSE: &str = "--verbose";

pub fn run() -> Result<()> {
    use config::Config;

    let args: Vec<String> = std::env::args().collect();

    if is_debug() || args.contains(&FLAG_VERBOSE.to_string()) {
        if let Err(e) = simple_logger::init() {
            eprintln!("Error initializing logger\n {}", e);
        }
    }

    let config = Config::from_args(&args)?;
    for e in config.requests.endpoints.iter() {
        info!("Fetching from: {}", &e.url);

        match network::make_http_request(&config, &e) {
            Ok(response) => {
                info!("{} :: SUCCESS", &e.url);
                let dst_file = files::create_dst_file(&config.dst, &e.url)?;
                files::write_response_to_file(dst_file, response)?;
            }
            Err(err) => error!("Error fetching from: {}\n{}", &e.url, err),
        }
    }

    Ok(())
}

fn is_debug() -> bool {
    return cfg![debug_assertions]
}
