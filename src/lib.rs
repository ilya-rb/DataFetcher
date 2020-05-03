#[macro_use]
extern crate log;
extern crate simple_logger;

mod config;
mod errors;
mod files;
mod network;
mod types;
mod yaml_config;

use types::Result;

const FLAG_VERBOSE: &str = "--verbose";
const FLAG_FORCE: &str = "--force";

pub fn run() -> Result<()> {
    use config::Config;

    let args: Vec<String> = std::env::args().map(|arg| arg.to_lowercase()).collect();

    if is_debug() || args.contains(&FLAG_VERBOSE.to_string()) {
        if let Err(e) = simple_logger::init_with_level(log::Level::Info) {
            eprintln!("Error initializing logger\n {}", e);
        }
    }

    let config = Config::from_args(&args)?;

    let mut executed_request_count = 0u32;
    let mut skipped_request_count = 0u32;

    for e in config.requests.endpoints.iter() {
        let dst_file = files::create_dst_file(&config.dst, &e.url)?;

        if files::is_file_exists(dst_file.file_name.as_str())
            && !args.contains(&FLAG_FORCE.to_string())
        {
            skipped_request_count += 1;
            continue;
        }

        info!("{} :: {}", &e.method, &e.url);

        match network::make_http_request(&config, &e) {
            Ok(response) => {
                files::write_response_to_file(
                    dst_file,
                    response.content_type.get_file_extension(),
                    response.response_text,
                )?;
                executed_request_count += 1;
            }
            Err(err) => error!("Error fetching from: {}\n{}", &e.url, err),
        }
    }

    info!(
        "Done: executed: {}, skipped: {}",
        executed_request_count, skipped_request_count
    );
    Ok(())
}

fn is_debug() -> bool {
    cfg![debug_assertions]
}
