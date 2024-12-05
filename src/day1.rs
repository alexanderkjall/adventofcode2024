use serde::de::{self, StdError};
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, MulAssign};
use std::{fmt, fs};

pub struct Day1Input {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl<'de> de::Deserialize<'de> for Day1Input {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(Day1InputVisitor)
    }
}

#[derive(Debug)]
pub struct MyError {
    err_string: String,
}

impl MyError {
    fn custom<T>(msg: T) -> Self
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

pub struct Day1InputVisitor;

impl Day1InputVisitor {
    // Look at the first character in the input without consuming it.
    fn peek_char(&self, input: &str) -> Result<char, MyError> {
        input.chars().next().ok_or(MyError::custom("Eof"))
    }

    // Consume the first character in the input.
    fn next_char<'a>(&self, input: &'a str) -> Result<(char, &'a str), MyError> {
        let ch = self.peek_char(input)?;
        let input = &input[ch.len_utf8()..];
        Ok((ch, input))
    }

    fn parse_space<'a>(&self, input: &'a str) -> Result<&'a str, MyError> {
        if input.starts_with(" ") {
            let num_space = input.chars().take_while(|c| *c == ' ').count();
            let input = &input[num_space..];
            Ok(input)
        } else {
            Err(MyError::custom("ExpectedSpace"))
        }
    }

    fn parse_newline<'a>(&self, input: &'a str) -> Result<&'a str, MyError> {
        if let Some(input) = input.strip_prefix("\n") {
            Ok(input)
        } else {
            Err(MyError::custom("ExpectedNewLine"))
        }
    }

    fn parse_unsigned<'a, T>(&self, input: &'a str) -> Result<(T, &'a str), MyError>
    where
        T: AddAssign<T> + MulAssign<T> + From<u8>,
    {
        let (ch, mut input) = self.next_char(input)?;
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
}

impl<'de> de::Visitor<'de> for Day1InputVisitor {
    type Value = Day1Input;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str(
            "two numbers separated by space, repeated on new lines arbitrary number of times",
        )
    }

    fn visit_str<E>(self, input: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut input: &str = input;
        let mut left = Vec::<u32>::new();
        let mut right = Vec::<u32>::new();

        while let Ok(c) = self.peek_char(input) {
            if c.is_ascii_digit() {
                let (num, new_input) = self
                    .parse_unsigned(input)
                    .map_err(|e| de::Error::custom(e))?;
                left.push(num);
                let new_input = self
                    .parse_space(new_input)
                    .map_err(|e| de::Error::custom(e))?;
                let (num, new_input) = self
                    .parse_unsigned(new_input)
                    .map_err(|e| de::Error::custom(e))?;
                right.push(num);
                let new_input = self
                    .parse_newline(new_input)
                    .map_err(|e| de::Error::custom(e))?;
                input = new_input;
            } else {
                break;
            }
        }

        Ok(Day1Input { left, right })
    }
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = MyError;

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char string
        byte_buf option unit unit_struct newtype_struct tuple tuple_struct
        seq map struct enum identifier ignored_any bytes
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::custom(
            "unsupported type provided to deserializer, only str is supported",
        ))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_str::<Self::Error>(self.input)
    }
}

pub fn deserialize<'a, T: de::Deserialize<'a>>(input: &'a str) -> Result<T, MyError> {
    let mut deserializer = Deserializer::from_str(input);
    let t = T::deserialize(&mut deserializer)?;

    Ok(t)
}

pub fn calculate() -> anyhow::Result<(i32, u64)> {
    let mut input: Day1Input = deserialize(&fs::read_to_string("input/day1")?)?;

    input.left.sort();
    input.right.sort();

    let mut distance = 0;
    for (l, r) in input.left.iter().zip(input.right.iter()) {
        distance += if l > r { l - r } else { r - l };
    }

    let mut similarity: u64 = 0;
    for l in input.left.iter() {
        similarity += (*l as u64) * input.right.iter().filter(|r| **r == *l).count() as u64;
    }
    Ok((distance as i32, similarity))
}
