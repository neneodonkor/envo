use std::{error, fmt};
use std::fmt::Formatter;


// -------------------------------------------------------------------------------------------------
// AggregateError is an aggregated error wrapper to combine gathered errors.
// This allows either to display all errors or convert them individually
// List of the available errors
// ParseError
// NotStructPtrError
// NoParserError
// NoSupportedTagOptionError
// VarIsNotSetError
// EmptyVarError
// LoadFileContentError
// ParseValueError
#[derive(Debug)]
pub struct AggregateError {
    pub errors: Vec<Box<dyn error::Error>>
}

impl AggregateError {
    pub fn new(init_err: Box<dyn error::Error>) -> Self {
        AggregateError {
            errors: vec![init_err]
        }
    }

    pub fn is<T: error::Error + 'static>(&self) -> bool {
        self.errors.iter().any(|e| e.downcast_ref::<T>().is_some())
    }
}

impl fmt::Display for AggregateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msgs: Vec<String> = self.errors.iter().map(|e| e.to_string()).collect();
        write!(f, "env: {};", msgs.join("; "))
    }
}

impl error::Error for AggregateError {}

// -------------------------------------------------------------------------------------------------
// ParseError occurs when it's impossible to convert the value for given type.
#[derive(Debug)]
pub struct ParseError {
    pub name: String,
    pub type_name: String,
    pub err: Box<dyn error::Error>
}

impl ParseError {
    pub fn new(name: String, type_name: String, err: Box<dyn error::Error>) -> Self {
        ParseError {
            name,
            type_name,
            err
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "parse error on field {:?} of type {:?}: {}", self.name, self.type_name, self.err)
    }
}

impl error::Error for ParseError {}

// -------------------------------------------------------------------------------------------------
// NotStructPtrError occurs when pass something that is not a pointer to a struct to Parse.
#[derive(Debug)]
pub struct NotStructPtrError;

impl fmt::Display for NotStructPtrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "expected a pointer to a Struct")
    }
}

impl error::Error for NotStructPtrError {}

// -------------------------------------------------------------------------------------------------
// NoParserError occurs when there is no parser provided for given type.
#[derive(Debug)]
pub struct NoParserError {
    pub name: String,
    pub type_name: String
}

impl NoParserError {
    pub fn new(name: String, type_name: String) -> Self {
        NoParserError {
            name,
            type_name
        }
    }
}

impl fmt::Display for NoParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "no parser found for field {:?} of type {:?}", self.name, self.type_name)
    }
}

impl error::Error for NoParserError {}

// -------------------------------------------------------------------------------------------------
// NoSupportedTagOptionError occurs when the given tag is not supported.
// Built-in supported tags: "", "file", "required", "unset", "notEmpty",
// "expand", "envDefault", and "envSeparator".
#[derive(Debug)]
pub struct NoSupportedTagOptionError {
    pub tag: String
}

impl NoSupportedTagOptionError {
    pub fn new(tag: String) -> Self {
        NoSupportedTagOptionError {
            tag
        }
    }
}

impl fmt::Display for NoSupportedTagOptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "tag option {:?} not supported", self.tag)
    }
}

impl error::Error for NoSupportedTagOptionError {}

// -------------------------------------------------------------------------------------------------
// VarIsNotSetError occurs when the required variable is not set.
#[derive(Debug)]
pub struct VarIsNotSetError {
    pub key: String
}

impl VarIsNotSetError {
    pub fn new(key: String) -> Self {
        VarIsNotSetError {
            key
        }
    }
}

impl fmt::Display for VarIsNotSetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "required environment variable {:?} is not set", self.key)
    }
}

impl error::Error for VarIsNotSetError {}

// -------------------------------------------------------------------------------------------------
// EmptyVarError occurs when the variable which must be not empty is existing but has an empty value
#[derive(Debug)]
pub struct EmptyVarError {
    pub key: String
}

impl EmptyVarError {
    pub fn new(key: String) -> Self {
        EmptyVarError {
            key
        }
    }
}

impl fmt::Display for EmptyVarError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "environment variable {:?} should not be empty", self.key)
    }
}

impl error::Error for EmptyVarError {}

// -------------------------------------------------------------------------------------------------
// LoadFileContentError occurs when it's impossible to load the value from the file.
#[derive(Debug)]
pub struct LoadFileContentError {
    pub filename: String,
    pub key: String,
    pub err: Box<dyn error::Error>
}

impl LoadFileContentError {
    pub fn new(filename: String, key: String, err: Box<dyn error::Error>) -> Self {
        LoadFileContentError {
            filename,
            key,
            err
        }
    }
}

impl fmt::Display for LoadFileContentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "could not load content of file {:?} from the variable {:?} : {:?}", self
            .filename, self.key, self.err)
    }
}

impl error::Error for LoadFileContentError {}

// -------------------------------------------------------------------------------------------------
// ParseValueError occurs when it's impossible to convert value using given parser.
#[derive(Debug)]
pub struct ParseValueError {
    pub msg: String,
    pub err: Box<dyn error::Error>
}

impl ParseValueError {
    pub fn new(msg: String, err: Box<dyn error::Error>) ->  Self {
        ParseValueError {
            msg,
            err
        }
    }
}

impl fmt::Display for ParseValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.msg, self.err)
    }
}

impl error::Error for ParseValueError {}