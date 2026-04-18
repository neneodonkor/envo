// Package env is a simple, zero-dependencies library to parse environment
// variables into structs.
// This is the Rust version.

/*
    Go uses reflect.Kind as the key — a runtime type identifier — which Rust doesn't have. In Rust,
    types are resolved at compile time, so you don't need a map like this at all.
    Instead, the idiomatic Rust approach is to use a trait.

    Go Code:
    var defaultBuiltInParsers = map[reflect.Kind]ParserFunc{
        reflect.Bool: func(v string) (interface{}, error) {
            return strconv.ParseBool(v)
        },
        reflect.String: func(v string) (interface{}, error) {
            return v, nil
        },
        reflect.Int: func(v string) (interface{}, error) {
            i, err := strconv.ParseInt(v, 10, 32)
            return int(i), err
        },
        reflect.Int16: func(v string) (interface{}, error) {
            i, err := strconv.ParseInt(v, 10, 16)
            return int16(i), err
        },
        reflect.Int32: func(v string) (interface{}, error) {
            i, err := strconv.ParseInt(v, 10, 32)
            return int32(i), err
        },
    }
*/
use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use url::Url;

pub trait Parser: Sized {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>>;
}

impl Parser for bool {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<bool>()?)
    }
}

impl Parser for String {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.to_string())
    }
}

impl Parser for i8 {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<i8>()?)
    }
}

impl Parser for i16 {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<i16>()?)
    }
}

impl Parser for i32 {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<i32>()?)
    }
}

impl Parser for i64 {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<i64>()?)
    }
}

impl Parser for i128 {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<i128>()?)
    }
}

impl Parser for u8 {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<u8>()?)
    }
}

impl Parser for u16 {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<u16>()?)
    }
}

impl Parser for u32 {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<u32>()?)
    }
}

impl Parser for u64 {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<u64>()?)
    }
}

impl Parser for u128 {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<u128>()?)
    }
}

impl Parser for f32 {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<f32>()?)
    }
}

impl Parser for f64 {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(s.parse::<f64>()?)
    }
}

impl Parser for Url {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Url::parse(s)?)
    }
}

impl Parser for std::time::Duration {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(duration_str::parse(s.to_lowercase())?)
    }
}

/*
    One thing to keep in mind — Go's interface{} and Rust's Box<dyn Any> are similar but not
    identical. In Rust, downcasting from Box<dyn Any> back to a concrete type requires calling
    .downcast::<T>(), whereas Go does it with a type assertion value.(ConcreteType). But for just
    defining the type alias, what you have is correct.
    type OnSetFn func(tag string, value interface{}, isDefault bool)
*/
type OnSetFn = fn(tag: &str, value: Box<dyn Any>, is_default: bool);

// Might need procedural macros (proc_macro), since it is difficult to port the reflection-heavy
// parts of the Go code.

/*
    type processFieldFn func(
        refField reflect.Value,
        refTypeField reflect.StructField,
        opts Options,
        fieldParams FieldParams,
    ) error
*/
type ProcessFieldFn = fn(
    field_value: &mut dyn Any,
    field_name: &str,
    opts: &Options,
    field_params: &FieldParams,
) -> Result<(), Box<dyn Error>>;

pub struct Options {
    // Environment keys and values that will be accessible for the service.
    pub environment: HashMap<String, String>,

    // tag_name specifies another tag name to use rather than the default 'env'.
    pub tag_name: String,

    // prefix_tag_name specifies another prefix tag name to use rather than the default 'envPrefix'.
    pub prefix_tag_name: String,

    // default_value_tag_name specifies another default tag name to use rather than the default 'envDefault'.
    pub default_value_tag_name: String,

    // required_if_no_def automatically sets all fields as required if they do not
    // declare 'envDefault'.
    pub required_if_no_def: bool,

    // on_set allows to run a function when a value is set.
    pub on_set: Option<OnSetFn>,

    // prefix define a prefix for every key.
    pub prefix: String,

    // use_fieldname_by_default defines whether `env` should use the field
    // name by default if the `env` key is missing.
    // Note that the field name will be "converted" to conform with environment
    // variable names conventions.
    pub use_fieldname_by_default: bool,

    // set_defaults_for_zero_values_only defines whether to set defaults for zero values
    // If the `env` variable for the value is not set, `envDefault` is set, and the value is not a
    // zero value for type, then set_defaults_for_zero_values_only=true. The value from
    // `envDefault` will be ignored. Useful for mixing default values from `envDefault` and struct
    // initialization
    pub set_defaults_for_zero_values_only: bool,

    // Custom parse functions for different types.
    pub func_map: HashMap<TypeId, Box<dyn Fn(&str) -> Result<Box<dyn Any>, Box<dyn Error>>>>,

    // Used internally. maps the env variable key to its resolved string value.
    // (for env var expansion)
    raw_env_vars: HashMap<String, String>
}

impl Options {
    fn get_raw_env(&self, s: &str) -> String {
        let value = self.raw_env_vars.get(s).map_or("", |s| s.as_str());

        let value = if value.is_empty() {
            self.environment.get(s).map_or("", |s| s.as_str())
        } else {
            value
        };

        shellexpand::env_with_context(value, |var|  {
            Ok(Some(self.get_raw_env(var)))
        }).unwrap_or_else(|_| Cow::from(value))
          .to_string()
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            tag_name: "env".to_string(),
            prefix_tag_name: "envPrefix".to_string(),
            default_value_tag_name: "envDefault".to_string(),
            environment: to_map(std::env::vars().collect()),
            func_map: HashMap::new(),
            raw_env_vars: HashMap::new(),
            ..Default::default()
        }
    }
}

pub fn to_map(env: Vec<(String, String)>) -> HashMap<String, String> {
    env.into_iter().collect()
}
pub struct  FieldParams {
    pub own_key: String,
    pub key: String,
    pub default_value: String,
    pub has_default_value: bool,
    pub required: bool,
    pub load_file: bool,
    pub unset: bool,
    pub not_empty: bool,
    pub expand: bool,
    pub init: bool,
    pub ignored: bool
}