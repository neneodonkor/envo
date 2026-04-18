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
use std::io;
use url::Url;
use crate::{EmptyVarError, ParseError, ParseValueError, VarIsNotSetError};

// This is the implementation of the ParserFunc in Go
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

impl Parser for chrono_tz::Tz {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        // Simple version
        // Ok(s.parse::<chrono_tz::Tz>()?)
        s.parse::<chrono_tz::Tz>().map_err(|e| {
            Box::new(ParseValueError {
                msg: "unable to parse location".to_string(),
                err: Box::new(io::Error::new(io::ErrorKind::Other, e.to_string()))
            }) as Box<dyn Error>
        })
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

#[derive(Clone)]
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
    pub fn get_raw_env(&self, s: &str) -> String {
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

    // Merges user-provided options into the default options.
    // Only non-zero/non-empty fields from the user-provided options
    // will override the defaults. FuncMap is merged rather than overwritten,
    // allowing custom parsers to extend the built-in ones.
    pub fn custom_options(self) -> Self {
        let mut default = Options::default();

        if !self.tag_name.is_empty() { default.tag_name = self.tag_name; }
        if !self.prefix_tag_name.is_empty() { default.prefix_tag_name = self.prefix_tag_name; }
        if !self.prefix.is_empty() { default.prefix = self.prefix; }
        if !self.default_value_tag_name.is_empty() {
            default.default_value_tag_name = self.default_value_tag_name;
        }

        if self.required_if_no_def {
            default.required_if_no_def = true;
        }

        if self.use_fieldname_by_default {
            default.use_fieldname_by_default = true;
        }

        if self.set_defaults_for_zero_values_only {
            default.set_defaults_for_zero_values_only = true;
        }

        if !self.environment.is_empty() {
            default.environment = self.environment;
        }

        if self.on_set.is_some() {
            default.on_set = self.on_set;
        }

        // func_map is special: merge instead of overwrite.
        default.func_map.extend(self.func_map);

        default
    }

    // Returns Options with Slice Environment Prefix
    pub fn options_with_slice_env_prefix(&self, index: usize) -> Self {
        Options {
            prefix: format!("{}{}_", self.prefix, index),
            ..self.clone()
        }
    }

    pub fn options_with_env_prefix(&self, prefix_value: &str) -> Self {
        Options {
            prefix: self.prefix.clone() + prefix_value,
            ..self.clone()
        }
    }
}

// Transformed the Go defaultOptions function into an impl block
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

pub trait FromEnv: Sized {
    fn from_env(opts: &Options) -> Result<Self, Box<dyn Error>>;
}

pub fn parse_internal<T: FromEnv>(opts: Options) -> Result<T, Box<dyn Error>> {
    T::from_env(&opts)
}

pub fn parse_field<T: Parser>(key: &str, opts: &Options) -> Result<T, Box<dyn Error>> {
    // 1. Look up the env var value
    let raw_value = opts.environment.get(key);

    // 2. Check if required
    if raw_value.is_none() {
        return Err(Box::new(VarIsNotSetError {
            key: key.to_string()
        }));
    }

    // 3. Check if empty
    let raw_value = raw_value.unwrap();
    if raw_value.is_empty() {
        return Err(Box::new(EmptyVarError {
            key: key.to_string()
        }));
    }

    // 4. Parse into target type using Parser trait
    T::parse(raw_value).map_err(|e| {
        Box::new(ParseError {
            name: key.to_string(),
            type_name: std::any::type_name::<T>().to_string(),
            err: e,
        }) as Box<dyn Error>
    })
}

/*pub fn parse<T>(v: &mut T) -> Result<(), Box<dyn Error>> {
    todo!()
}
pub fn parse_with_options<T>(v: &mut T, opts: Options) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn parse_as<T: Default>() -> Result<T, Box<dyn Error>> {
    todo!()
}
pub fn parse_as_with_options<T: Default>(opts: Options) -> Result<T, Box<dyn Error>> {
    todo!()
}*/

pub fn must<T>(result: Result<T, Box<dyn Error>>) -> T {
    result.unwrap_or_else(|e| panic!("{}", e))
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

