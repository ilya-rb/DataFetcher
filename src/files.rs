use crate::errors::Error;
use crate::types::Result;

pub const EXT_TEXT: &str = "txt";
pub const EXT_HTML: &str = "html";
pub const EXT_JSON: &str = "json";

#[derive(Debug)]
pub struct FileToSave {
    pub file_path: String,
    pub file_name: String,
}

pub fn create_dst_file(root_folder_path: &str, url: &str) -> Result<FileToSave> {
    use crate::errors::AppErrorType::UrlParseError;
    use crate::errors::Error::AppError;
    use reqwest::Url;

    let url = Url::parse(&url).map_err(|_| AppError(UrlParseError, "Unable to parse http url"))?;
    let url_path: Vec<&str> = url.path_segments().unwrap().collect();

    // Using last part of the url as a file name
    let (file_name, url_path) = url_path.split_last().unwrap();

    // Trim ending slash if needed
    let root_folder_path = if root_folder_path.ends_with('/') {
        &root_folder_path[0..root_folder_path.len() - 1]
    } else {
        root_folder_path
    };

    let file_path = format!("{}/{}", root_folder_path, url_path.join("/"));
    let file_name = format!("{}/{}", file_path, file_name);

    Ok(FileToSave {
        file_path,
        file_name,
    })
}

pub fn is_file_exists(file_name: &str) -> bool {
    vec![EXT_TEXT, EXT_HTML, EXT_JSON]
        .iter()
        .map(|ext| format!("{}.{}", file_name, ext))
        .any(|path| std::path::Path::new(&path).exists())
}

pub fn write_response_to_file(
    dst: FileToSave,
    file_extension: &str,
    response: String,
) -> Result<()> {
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    if let Err(e) = fs::create_dir_all(dst.file_path) {
        // Ignore AlreadyExists error as we probably re-downloading
        // existing response with --force flag.
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(Error::IoError(e));
        }
    };

    let mut dst_file = File::create(format!("{}.{}", dst.file_name, file_extension))?;
    dst_file.write_all(&response.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_dst_file_should_return_correct_file() {
        let root_folder_path = "dst/root/";
        let url = "https://api.com/endpoint/path";
        let result = create_dst_file(&root_folder_path, &url).unwrap();

        assert_eq!(result.file_path, "dst/root/endpoint");
        assert_eq!(result.file_name, "dst/root/endpoint/path");
    }
}
