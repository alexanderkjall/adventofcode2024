use crate::error::MyError;
use crate::parsing::{parse_newline, parse_space, parse_token, parse_unsigned, peek_char};
use serde::de::{self};
use std::fmt::Formatter;
use std::fmt;

pub struct Day3Input {
    multipliers: Vec<(u32, u32)>,
}

impl<'de> de::Deserialize<'de> for Day3Input {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(Day3InputVisitor)
    }
}

pub struct Day3InputVisitor;

impl<'de> de::Visitor<'de> for Day3InputVisitor {
    type Value = Day3Input;

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
        let mut multipliers = Vec::<(u32, u32)>::new();
        let mut found = false;

        while let Ok(c) = peek_char(input) {
            if c == 'm' {
                let (hit, new_input) = parse_token(input, "mul(");
                if hit {
                    if let Ok((first, new_input)) = parse_unsigned(new_input).map_err(|e| <de::value::Error as de::Error>::custom(e)) {
                        let (hit, new_input) = parse_token(new_input, ",");
                        if hit {
                            if let Ok((second, new_input)) = parse_unsigned(new_input).map_err(|e| <de::value::Error as de::Error>::custom(e)) {
                                let (hit, new_input) = parse_token(new_input, ")");
                                if hit {
                                    found = true;
                                    input = new_input;
                                    multipliers.push((first, second));
                                }
                            }
                        }
                    }
                }
            }
            if !found {
                input = &input[1..];
            }
            found = false;
        }

        Ok(Day3Input { multipliers })
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
    let input: Day3Input = deserialize(input)?;

    let sum = input.multipliers.iter().fold(0, |init, (f, s)| init + f * s);

    Ok((sum as i32, 1))
}
