use crate::error::MyError;
use crate::parsing::{parse_newline, parse_space, parse_unsigned, peek_char};
use serde::de::{self};
use std::fmt::Formatter;
use std::fmt;

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

pub struct Day1InputVisitor;

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

        while let Ok(c) = peek_char(input) {
            if c.is_ascii_digit() {
                let (num, new_input) = parse_unsigned(input).map_err(|e| de::Error::custom(e))?;
                left.push(num);
                let new_input = parse_space(new_input).map_err(|e| de::Error::custom(e))?;
                let (num, new_input) =
                    parse_unsigned(new_input).map_err(|e| de::Error::custom(e))?;
                right.push(num);
                let new_input = parse_newline(new_input).map_err(|e| de::Error::custom(e))?;
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

pub fn calculate(input: &str) -> anyhow::Result<(i32, u64)> {
    let mut input: Day1Input = deserialize(input)?;

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
