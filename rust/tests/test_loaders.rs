#![deny(
    anonymous_parameters,
    bad_style,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    unused_qualifications,
)]

extern crate url;

use rust_swagger_validator::loaders::*;
use std::path;

macro_rules! panic_with_expected_loader_error {
    ($expression_to_panic:expr, $expected_enum_type:tt ) => {
        panic_with_expected_loader_error!(
            $expression_to_panic,
            $expected_enum_type,
            |inner_error| {
                // Small hack to ensure that the closure returns a Result object that will always return Ok(1)
                match Option::from(inner_error) {
                    Some(_) => Ok(1),
                    None => Err(1),
                }
            }
        );
    };

    ($expression_to_panic:expr, $expected_enum_type:tt, $inner_error_check:expr) => {
        let throw_panic = || {
            panic!(
                "{} is not panicking as expected",
                stringify!($expression_to_panic)
            )
        };

        match $expression_to_panic {
            Err(error) => match error {
                LoaderError::$expected_enum_type(inner_error) => {
                    let inner_check = $inner_error_check(inner_error);
                    if inner_check.is_err() {
                        throw_panic()
                    }
                }
                _ => throw_panic(),
            },
            _ => throw_panic(),
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
    let yaml_value = load_from_path("test-data/json-files-loaders-tests/valid.json", None).unwrap();

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
    let url = url::Url::from_file_path(absolute_path).unwrap();
    let json_value = load_from_url(url.as_str(), Option::from(Format::JSON)).unwrap();

    let json_string = json_value.get("key").unwrap();
    assert_eq!(json_string.as_str().unwrap(), "value");
}

#[test]
fn test_load_from_url_yaml_format_valid_yaml() {
    let path_str = "test-data/yaml-files-loaders-tests/valid.yaml";
    let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
    let url = url::Url::from_file_path(absolute_path).unwrap();
    let json_value = load_from_url(url.as_str(), Option::from(Format::YAML)).unwrap();

    let json_string = json_value.get("key").unwrap();
    assert_eq!(json_string.as_str().unwrap(), "value");
}

#[test]
fn test_load_from_url_no_format_valid_json() {
    let path_str = "test-data/json-files-loaders-tests/valid.json";
    let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
    let url = url::Url::from_file_path(absolute_path).unwrap();
    let json_value = load_from_url(url.as_str(), None).unwrap();

    let json_string = json_value.get("key").unwrap();
    assert_eq!(json_string.as_str().unwrap(), "value");
}

#[test]
fn test_load_from_url_invalid_url() {
    panic_with_expected_loader_error!(load_from_url("this-is-an-invalid-url", None), InvalidURL);
}

#[test]
fn test_load_from_url_with_timeout_json_format_valid_json() {
    let path_str = "test-data/json-files-loaders-tests/valid.json";
    let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
    let url = url::Url::from_file_path(absolute_path).unwrap();
    let json_value =
        load_from_url_with_timeout(url.as_str(), 1, Option::from(Format::JSON)).unwrap();

    let json_string = json_value.get("key").unwrap();
    assert_eq!(json_string.as_str().unwrap(), "value");
}

#[test]
fn test_load_from_url_with_timeout_yaml_format_valid_yaml() {
    let path_str = "test-data/yaml-files-loaders-tests/valid.yaml";
    let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
    let url = url::Url::from_file_path(absolute_path).unwrap();
    let json_value =
        load_from_url_with_timeout(url.as_str(), 1, Option::from(Format::YAML)).unwrap();

    let json_string = json_value.get("key").unwrap();
    assert_eq!(json_string.as_str().unwrap(), "value");
}

#[test]
fn test_load_from_url_with_timeout_no_format_valid_json() {
    let path_str = "test-data/json-files-loaders-tests/valid.json";
    let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
    let url = url::Url::from_file_path(absolute_path).unwrap();
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
fn test_load_from_url_with_syntattically_invalid_url() {
    panic_with_expected_loader_error!(
        load_from_url_with_timeout("http:/caca", 1, None),
        InvalidURL,
        |inner_error| match inner_error {
            UrlError::SyntaxViolation(_) => Ok(1),
            _ => Err(1),
        }
    );
}

#[test]
fn test_load_from_url_with_timeout_unfetchable_url() {
    panic_with_expected_loader_error!(
        load_from_url_with_timeout("scheme://not-existing-url.local", 200, None),
        FetchURLFailed
    );
}
