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

    #[inline]
    fn load_from_path(&self, path: &str) -> Result<serde_json::Value, LoaderError> {
        let mut content = String::new();
        let _ = fs::File::open(&path::Path::new(path))?.read_to_string(&mut content)?;
        self.load_from_string(content)
    }

    #[inline]
    fn load_from_url(&self, url: &str) -> Result<serde_json::Value, LoaderError> {
        self.load_from_url_with_timeout(url, 30_000)
    }

    #[inline]
    fn load_from_url_with_timeout(
        &self,
        url: &str,
        timeout_ms: u64,
    ) -> Result<serde_json::Value, LoaderError> {
        let url = url::Url::parse(url)?;
        if url.scheme() == "file" {
            self.load_from_path(url.path())
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
    #[inline]
    fn load_from_string(&self, content: String) -> Result<serde_json::Value, LoaderError> {
        match serde_json::from_str(&content) {
            Ok(value) => Ok(value),
            Err(serde_error) => Err(LoaderError::JSONError(serde_error)),
        }
    }
}

impl Loader for YAMLLoader {
    #[inline]
    fn load_from_string(&self, content: String) -> Result<serde_json::Value, LoaderError> {
        match serde_yaml::from_str(&content) {
            Ok(value) => Ok(value),
            Err(serde_error) => Err(LoaderError::YAMLError(serde_error)),
        }
    }
}

#[inline]
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

#[inline]
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

#[inline]
pub fn load_from_url(url: &str, format: Option<Format>) -> Result<serde_json::Value, LoaderError> {
    match format {
        None => Format::YAML, // TODO: make it smarter?
        Some(format) => format,
    }.get_loader()
    .load_from_url(url)
}

#[inline]
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

#[cfg(test)]
mod tests {
    use super::load_from_path;
    use super::load_from_string;
    use super::load_from_url;
    use super::load_from_url_with_timeout;
    use super::Format;
    use super::LoaderError;
    use std::path;

    macro_rules! panic_with_expected_loader_error {
        ($expression_to_panic:expr, $expected_enum_type:tt ) => {
            match $expression_to_panic {
                Err(error) => match error {
                    LoaderError::$expected_enum_type(_inner_error) => {}
                    _ => panic!(
                        "{} is not panicking as expected",
                        stringify!($expression_to_panic)
                    ),
                },
                _ => panic!(
                    "{} is not panicking as expected",
                    stringify!($expression_to_panic)
                ),
            }
        };
    }

    #[test]
    fn test_load_from_string_json_format_valid_content() {
        let json_content = r#"
            {
                "key_string": "value",
                "key_integer": 1,
                "key_boolean": true
            }
        "#;
        let json_value =
            load_from_string(String::from(json_content), Option::from(Format::JSON)).unwrap();

        let json_string = json_value.get("key_string").unwrap();
        assert_eq!(json_string.as_str().unwrap(), "value");

        let json_integer = json_value.get("key_integer").unwrap();
        assert_eq!(json_integer.as_i64().unwrap(), 1);

        let json_boolean = json_value.get("key_boolean").unwrap();
        assert_eq!(json_boolean.as_bool().unwrap(), true);
    }

    #[test]
    fn test_load_from_string_yaml_format_valid_content() {
        let yaml_content = r#"
            key_string: value
            key_integer: 1
            key_boolean: true
        "#;
        let yaml_value =
            load_from_string(String::from(yaml_content), Option::from(Format::YAML)).unwrap();

        let yaml_string = yaml_value.get("key_string").unwrap();
        assert_eq!(yaml_string.as_str().unwrap(), "value");

        let yaml_integer = yaml_value.get("key_integer").unwrap();
        assert_eq!(yaml_integer.as_i64().unwrap(), 1);

        let yaml_boolean = yaml_value.get("key_boolean").unwrap();
        assert_eq!(yaml_boolean.as_bool().unwrap(), true);
    }

    #[test]
    fn test_load_from_string_no_format_valid_json_content() {
        let json_content = r#"
            {
                "key_string": "value",
                "key_integer": 1,
                "key_boolean": true
            }
        "#;
        let yaml_value = load_from_string(String::from(json_content), None).unwrap();

        let yaml_string = yaml_value.get("key_string").unwrap();
        assert_eq!(yaml_string.as_str().unwrap(), "value");

        let yaml_integer = yaml_value.get("key_integer").unwrap();
        assert_eq!(yaml_integer.as_i64().unwrap(), 1);

        let yaml_boolean = yaml_value.get("key_boolean").unwrap();
        assert_eq!(yaml_boolean.as_bool().unwrap(), true);
    }

    #[test]
    fn test_load_from_string_json_format_invalid_content() {
        let json_content = r#"
            {
                "key_object": {
            }
        "#;
        panic_with_expected_loader_error!(
            load_from_string(String::from(json_content), Option::from(Format::JSON)),
            JSONError
        );
    }

    #[test]
    fn test_load_from_string_yaml_format_invalid_content() {
        let yaml_content = r#"
            key_object: {
        "#;
        panic_with_expected_loader_error!(
            load_from_string(String::from(yaml_content), Option::from(Format::YAML)),
            YAMLError
        );
    }

    #[test]
    fn test_load_from_string_no_format_invalid_json_content() {
        let json_content = r#"
            {
                "key_object": {
            }
        "#;
        panic_with_expected_loader_error!(
            load_from_string(String::from(json_content), None),
            YAMLError
        );
    }

    #[test]
    fn test_load_from_path_json_format_valid_json() {
        let json_value = load_from_path(
            "test-data/json-files-loaders-tests/valid.json",
            Option::from(Format::JSON),
        ).unwrap();

        let json_string = json_value.get("key").unwrap();
        assert_eq!(json_string.as_str().unwrap(), "value");
    }

    #[test]
    fn test_load_from_path_yaml_format_valid_yaml() {
        let yaml_value = load_from_path(
            "test-data/yaml-files-loaders-tests/valid.yaml",
            Option::from(Format::YAML),
        ).unwrap();

        let yaml_string = yaml_value.get("key").unwrap();
        assert_eq!(yaml_string.as_str().unwrap(), "value");
    }

    #[test]
    fn test_load_from_path_no_format_valid_json() {
        let yaml_value =
            load_from_path("test-data/json-files-loaders-tests/valid.json", None).unwrap();

        let json_string = yaml_value.get("key").unwrap();
        assert_eq!(json_string.as_str().unwrap(), "value");
    }

    #[test]
    fn test_load_from_path_file_not_found() {
        panic_with_expected_loader_error!(load_from_path("test-data/no-file-found", None), IOError);
    }

    #[test]
    fn test_load_from_url_json_format_valid_json() {
        let path_str = "test-data/json-files-loaders-tests/valid.json";
        let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
        let url = format!("file://{}", absolute_path.to_str().unwrap());
        let json_value = load_from_url(url.as_str(), Option::from(Format::JSON)).unwrap();

        let json_string = json_value.get("key").unwrap();
        assert_eq!(json_string.as_str().unwrap(), "value");
    }

    #[test]
    fn test_load_from_url_yaml_format_valid_yaml() {
        let path_str = "test-data/yaml-files-loaders-tests/valid.yaml";
        let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
        let url = format!("file://{}", absolute_path.to_str().unwrap());
        let json_value = load_from_url(url.as_str(), Option::from(Format::YAML)).unwrap();

        let json_string = json_value.get("key").unwrap();
        assert_eq!(json_string.as_str().unwrap(), "value");
    }

    #[test]
    fn test_load_from_url_no_format_valid_json() {
        let path_str = "test-data/json-files-loaders-tests/valid.json";
        let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
        let url = format!("file://{}", absolute_path.to_str().unwrap());
        let json_value = load_from_url(url.as_str(), None).unwrap();

        let json_string = json_value.get("key").unwrap();
        assert_eq!(json_string.as_str().unwrap(), "value");
    }

    #[test]
    fn test_load_from_url_invalid_url() {
        panic_with_expected_loader_error!(
            load_from_url("this-is-an-invalid-url", None),
            InvalidURL
        );
    }

    #[test]
    fn test_load_from_url_with_timeout_json_format_valid_json() {
        let path_str = "test-data/json-files-loaders-tests/valid.json";
        let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
        let url = format!("file://{}", absolute_path.to_str().unwrap());
        let json_value =
            load_from_url_with_timeout(url.as_str(), 1, Option::from(Format::JSON)).unwrap();

        let json_string = json_value.get("key").unwrap();
        assert_eq!(json_string.as_str().unwrap(), "value");
    }

    #[test]
    fn test_load_from_url_with_timeout_yaml_format_valid_yaml() {
        let path_str = "test-data/yaml-files-loaders-tests/valid.yaml";
        let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
        let url = format!("file://{}", absolute_path.to_str().unwrap());
        let json_value =
            load_from_url_with_timeout(url.as_str(), 1, Option::from(Format::YAML)).unwrap();

        let json_string = json_value.get("key").unwrap();
        assert_eq!(json_string.as_str().unwrap(), "value");
    }

    #[test]
    fn test_load_from_url_with_timeout_no_format_valid_json() {
        let path_str = "test-data/json-files-loaders-tests/valid.json";
        let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
        let url = format!("file://{}", absolute_path.to_str().unwrap());
        let json_value = load_from_url_with_timeout(url.as_str(), 1, None).unwrap();

        let json_string = json_value.get("key").unwrap();
        assert_eq!(json_string.as_str().unwrap(), "value");
    }

    #[test]
    fn test_load_from_url_with_timeout_invalid_url() {
        panic_with_expected_loader_error!(
            load_from_url_with_timeout("this-is-an-invalid-url", 1, None),
            InvalidURL
        );
    }

    #[test]
    fn test_load_from_url_with_timeout_unfetchable_url() {
        panic_with_expected_loader_error!(
            load_from_url_with_timeout("http://not-existing-url.local", 100, None),
            FetchURLFailed
        );
    }
}
