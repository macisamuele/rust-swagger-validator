#![deny(
    anonymous_parameters,
    bad_style,
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs, // TODO: add missing_docs to list of deny
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    unused_qualifications,
)]

extern crate serde_json;
extern crate url;
extern crate valico;

use self::serde_json::Value;
use self::url::Url;
use self::valico::json_schema;
use self::valico::json_schema::schema;
use loaders;
use loaders::load_from_path;
use loaders::load_from_url;
use std::fmt;
use std::io;
use std::path;

// FIXME: follow references during validation
// FIXME: support for custom formats
// FIXME: note that could be possible having custom
#[derive(Debug)]
pub struct SwaggerSchema {
    pub uri: Option<Url>,
    pub schema: schema::Schema,
}

enum_with_automatic_from_trait_implementation!(
    derive(Debug),
    SwaggerSchemaError,
    IOError(io::Error),
    InvalidURL(url::ParseError),
    LoaderError(loaders::LoaderError),
    SchemaError(schema::SchemaError),
    ValidationError(json_schema::ValidationState)
);

impl SwaggerSchema {
    pub fn new_from_url(url: &str) -> Result<Self, SwaggerSchemaError> {
        Self::new_from_content(load_from_url(url, None)?, Option::from(Url::parse(url)?))
    }

    pub fn new_from_path(path: &str) -> Result<Self, SwaggerSchemaError> {
        Self::new_from_content(
            load_from_path(path, None)?,
            Option::from(Url::from_file_path(path::Path::new(path).canonicalize()?).unwrap()),
        )
    }

    pub fn new_from_content(
        swagger_spec: Value,
        url: Option<Url>,
    ) -> Result<Self, SwaggerSchemaError> {
        let draft4_schema = load_from_path("schema/draft4.json", None)?;
        let swagger_20_schema = load_from_path("schema/swagger2.0.json", None)?;

        let mut new_scope = json_schema::Scope::new();
        let _ = new_scope.compile(draft4_schema, true)?;

        let scoped_schema = new_scope.compile_and_return(swagger_20_schema, false)?;

        let validation_state = scoped_schema.validate(&swagger_spec);
        if validation_state.is_valid() {
            let swagger_schema = schema::compile(
                swagger_spec,
                url.clone(),
                schema::CompilationSettings::new(&json_schema::keywords::default(), false),
            )?;
            Ok(Self {
                uri: url,
                schema: swagger_schema,
            })
        } else {
            Err(SwaggerSchemaError::ValidationError(validation_state))
        }
    }

    pub fn validation_state(self, object: &Value) -> json_schema::ValidationState {
        let scope = json_schema::Scope::new();
        let scoped_schema = schema::ScopedSchema::new(&scope, &self.schema);
        scoped_schema.validate(object)
    }
}

impl<'a> fmt::Display for SwaggerSchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Uri {:?}\n{:?}", self.uri, self.schema)
    }
}
