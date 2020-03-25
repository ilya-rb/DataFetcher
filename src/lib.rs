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
const FLAG_FORCE: &str = "--force";

pub fn run() -> Result<()> {
    use config::Config;
    use std::path::Path;

    let args: Vec<String> = std::env::args()
      .map(|arg| arg.to_lowercase())
      .collect();

    if is_debug() || args.contains(&FLAG_VERBOSE.to_string()) {
        if let Err(e) = simple_logger::init() {
            eprintln!("Error initializing logger\n {}", e);
        }
    }

    let config = Config::from_args(&args)?;
    
    let mut success_request_count = 0u32;
    let mut skipped_request_count = 0u32;

    for e in config.requests.endpoints.iter() {
        let dst_file = files::create_dst_file(&config.dst, &e.url)?;
        let dst_file_path = Path::new(&dst_file.file_name);

        if dst_file_path.exists() && !args.contains(&FLAG_FORCE.to_string()) {
            let file = std::fs::File::open(&dst_file_path)?;
            let metadata = file.metadata()?;
            
            // File already contain some response - skip downloading.
            if metadata.len() > 0  {
              skipped_request_count += 1;
              continue;
            }
        }
        
        info!("Fetching from: {}", &e.url);

        match network::make_http_request(&config, &e) {
          Ok(response) => {
            files::write_response_to_file(dst_file, response)?;
            info!("{} :: Success", &e.url);
            success_request_count += 1;
          },
          Err(err) => error!("Error fetching from: {}\n{}", &e.url, err),
        }
    }

    info!("Done: executed: {}, skipped: {}", success_request_count, skipped_request_count);
    Ok(())
}

fn is_debug() -> bool {
    cfg![debug_assertions]
}
