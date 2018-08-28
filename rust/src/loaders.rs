#![deny(
    anonymous_parameters,
    bad_style,
    missing_copy_implementations,
    missing_debug_implementations,
// missing_docs, // TODO: add missing_docs to list of deny
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    unused_qualifications,
)]

extern crate reqwest;
extern crate serde_json;
extern crate serde_yaml;
extern crate url;

use std::fs;
use std::io;
use std::io::prelude::Read;
use std::path;
use std::time::Duration;

#[derive(Debug)]
pub enum LoaderError {
    IOError(io::Error),
    InvalidURL(url::ParseError),
    FetchURLFailed(reqwest::Error),
    JSONError(serde_json::Error),
    YAMLError(serde_yaml::Error),
}
from_error_to_enum_variant!(io::Error, LoaderError, IOError);
from_error_to_enum_variant!(url::ParseError, LoaderError, InvalidURL);
from_error_to_enum_variant!(reqwest::Error, LoaderError, FetchURLFailed);

#[derive(Clone, Copy, Debug)]
pub enum Format {
    JSON,
    YAML,
}

pub trait Loader {
    fn load_from_string(&self, content: String) -> Result<serde_json::Value, LoaderError>;

    fn load_from_path(&self, path: &str) -> Result<serde_json::Value, LoaderError> {
        let mut content = String::new();
        let _ = fs::File::open(&path::Path::new(path))?.read_to_string(&mut content)?;
        self.load_from_string(content)
    }

    fn load_from_url(&self, url: &str) -> Result<serde_json::Value, LoaderError> {
        self.load_from_url_with_timeout(url, 30_000)
    }

    fn load_from_url_with_timeout(
        &self,
        url: &str,
        timeout_ms: u64,
    ) -> Result<serde_json::Value, LoaderError> {
        let url = url::Url::parse(url)?;
        if url.scheme() == "file" {
            // Using unwrap as we do assume that the url is valid
            self.load_from_path(url.to_file_path().unwrap().to_str().unwrap())
        } else {
            let mut client_builder = reqwest::Client::builder();
            let client = client_builder
                .gzip(true)
                .timeout(Duration::from_millis(timeout_ms))
                .build()?;
            self.load_from_string(client.get(url.as_ref()).send()?.text()?)
        }
    }
}

struct JSONLoader;
struct YAMLLoader;
impl Format {
    fn get_loader(self) -> Box<Loader> {
        match self {
            Format::YAML => Box::new(YAMLLoader),
            Format::JSON => Box::new(JSONLoader),
        }
    }
}

impl Loader for JSONLoader {
    fn load_from_string(&self, content: String) -> Result<serde_json::Value, LoaderError> {
        match serde_json::from_str(&content) {
            Ok(value) => Ok(value),
            Err(serde_error) => Err(LoaderError::JSONError(serde_error)),
        }
    }
}

impl Loader for YAMLLoader {
    fn load_from_string(&self, content: String) -> Result<serde_json::Value, LoaderError> {
        match serde_yaml::from_str(&content) {
            Ok(value) => Ok(value),
            Err(serde_error) => Err(LoaderError::YAMLError(serde_error)),
        }
    }
}

pub fn load_from_string(
    content: String,
    format: Option<Format>,
) -> Result<serde_json::Value, LoaderError> {
    match format {
        None => Format::YAML, // TODO: make it smarter?
        Some(format) => format,
    }.get_loader()
    .load_from_string(content)
}

pub fn load_from_path(
    path: &str,
    format: Option<Format>,
) -> Result<serde_json::Value, LoaderError> {
    match format {
        None => Format::YAML, // TODO: make it smarter?
        Some(format) => format,
    }.get_loader()
    .load_from_path(path)
}

pub fn load_from_url(url: &str, format: Option<Format>) -> Result<serde_json::Value, LoaderError> {
    match format {
        None => Format::YAML, // TODO: make it smarter?
        Some(format) => format,
    }.get_loader()
    .load_from_url(url)
}

pub fn load_from_url_with_timeout(
    url: &str,
    timeout_ms: u64,
    format: Option<Format>,
) -> Result<serde_json::Value, LoaderError> {
    match format {
        None => Format::YAML, // TODO: make it smarter?
        Some(format) => format,
    }.get_loader()
    .load_from_url_with_timeout(url, timeout_ms)
}
