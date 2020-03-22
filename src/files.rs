use crate::config::Config;
use crate::config;

pub fn write_response_to_file(config: &Config, response: String, url: &str) {
    use reqwest::Url;
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    // TODO: Handle unwraps here
    let url = Url::parse(&url).unwrap();
    let url_path: Vec<&str> = url.path_segments().unwrap().collect();

    let (file_name, url_path) = url_path.split_last().unwrap();

    let url_path = format!("{}/{}", config.dst, url_path.join("/"));
    let file_name = format!("{}/{}.{}", url_path, file_name, config::CONFIG_FILE_EXT);

    // TODO: Handle result here
    fs::create_dir_all(url_path);

    if let Ok(mut f) = File::create(file_name) {
        // TODO: Handle result here
        f.write_all(&response.as_bytes());
    }
}