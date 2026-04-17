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
        Ok(url::Url::parse(s)?)
    }
}

impl Parser for std::time::Duration {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(duration_str::parse(s.to_lowercase())?)
    }
}