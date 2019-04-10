#![deny(
    anonymous_parameters,
    bad_style,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    unused_qualifications
)]

extern crate url;

use rust_swagger_validator::swagger_schema::*;
use std::path;

#[test]
fn test_load_swagger_schema_from_path_with_valid_json_specs() {
    let _ = SwaggerSchema::new_from_path("test-data/json-valid-specs/swagger.json")
        .expect("Unexpected error");
}

#[test]
fn test_load_swagger_schema_from_path_with_valid_yaml_specs() {
    let _ = SwaggerSchema::new_from_path("test-data/yaml-valid-specs/swagger.yaml")
        .expect("Unexpected error");
}

#[test]
fn test_load_swagger_schema_from_path_with_invalid_json_specs() {
    let _ = SwaggerSchema::new_from_path("test-data/json-invalid-specs/swagger.json")
        .expect_err("Expected error");
}

#[test]
fn test_load_swagger_schema_from_path_with_invalid_yaml_specs() {
    let _ = SwaggerSchema::new_from_path("test-data/yaml-invalid-specs/swagger.yaml")
        .expect_err("Expected error");
}

#[test]
fn test_load_swagger_schema_from_url_with_valid_json_specs() {
    let path_str = "test-data/json-valid-specs/swagger.json";
    let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
    let url = url::Url::from_file_path(absolute_path).unwrap();

    let _ = SwaggerSchema::new_from_url(url.as_str()).expect("Unexpected error");
}

#[test]
fn test_load_swagger_schema_from_url_with_valid_yaml_specs() {
    let path_str = "test-data/yaml-valid-specs/swagger.yaml";
    let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
    let url = url::Url::from_file_path(absolute_path).unwrap();

    let _ = SwaggerSchema::new_from_url(url.as_str()).expect("Unexpected error");
}

#[test]
fn test_load_swagger_schema_from_url_with_invalid_json_specs() {
    let path_str = "test-data/json-invalid-specs/swagger.json";
    let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
    let url = url::Url::from_file_path(absolute_path).unwrap();

    let _ = SwaggerSchema::new_from_url(url.as_str()).expect_err("Expected error");
}

#[test]
fn test_load_swagger_schema_from_url_with_invalid_yaml_specs() {
    let path_str = "test-data/yaml-invalid-specs/swagger.yaml";
    let absolute_path = path::Path::new(path_str).canonicalize().unwrap();
    let url = url::Url::from_file_path(absolute_path).unwrap();

    let _ = SwaggerSchema::new_from_url(url.as_str()).expect_err("Expected error");
}
