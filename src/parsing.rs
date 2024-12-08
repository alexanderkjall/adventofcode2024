use crate::error::MyError;
use std::ops::{AddAssign, MulAssign};

// Look at the first character in the input without consuming it.
pub fn peek_char(input: &str) -> Result<char, MyError> {
    input.chars().next().ok_or(MyError::custom("Eof"))
}

// Consume the first character in the input.
pub fn next_char(input: &str) -> Result<(char, &str), MyError> {
    let ch = peek_char(input)?;
    let input = &input[ch.len_utf8()..];
    Ok((ch, input))
}

pub fn parse_space(input: &str) -> Result<&str, MyError> {
    if input.starts_with(" ") {
        let num_space = input.chars().take_while(|c| *c == ' ').count();
        let input = &input[num_space..];
        Ok(input)
    } else {
        Err(MyError::custom("ExpectedSpace"))
    }
}

pub fn parse_newline(input: &str) -> Result<&str, MyError> {
    if let Some(input) = input.strip_prefix("\n") {
        Ok(input)
    } else {
        Err(MyError::custom("ExpectedNewLine"))
    }
}

pub fn parse_unsigned<T>(input: &str) -> Result<(T, &str), MyError>
where
    T: AddAssign<T> + MulAssign<T> + From<u8>,
{
    let (ch, mut input) = next_char(input)?;
    let mut int = match ch {
        ch @ '0'..='9' => T::from(ch as u8 - b'0'),
        _ => {
            return Err(MyError::custom("ExpectedInteger"));
        }
    };
    loop {
        match input.chars().next() {
            Some(ch @ '0'..='9') => {
                input = &input[1..];
                int *= T::from(10);
                int += T::from(ch as u8 - b'0');
            }
            _ => {
                return Ok((int, input));
            }
        }
    }
}

pub fn parse_token<'a>(input: &'a str, token: &str) -> (bool, &'a str)
{
    if input.starts_with(token) {
        (true, &input[token.len()..])
    } else {
        (false, input)
    }
}
