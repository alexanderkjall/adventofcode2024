use serde::de;
use serde::de::StdError;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct MyError {
    err_string: String,
}

impl MyError {
    pub fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self {
            err_string: msg.to_string(),
        }
    }
}

impl StdError for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.err_string)
    }
}

impl de::Error for MyError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self {
            err_string: msg.to_string(),
        }
    }
}
