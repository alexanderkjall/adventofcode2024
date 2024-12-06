use crate::error::MyError;
use crate::parsing::{next_char, parse_unsigned, peek_char};
use serde::de::{self};
use std::fmt::Formatter;
use std::fmt;

pub struct Report {
    levels: Vec<u32>,
}
pub struct Day2Input {
    reports: Vec<Report>,
}

impl<'de> de::Deserialize<'de> for Day2Input {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(Day2InputVisitor)
    }
}

pub struct Day2InputVisitor;

impl<'de> de::Visitor<'de> for Day2InputVisitor {
    type Value = Day2Input;

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
        let mut reports = Vec::<Report>::new();

        while let Ok(c) = peek_char(input) {
            if c.is_ascii_digit() {
                let mut levels = Vec::<u32>::new();
                let (num, new_input) = parse_unsigned(input).map_err(|e| de::Error::custom(e))?;
                levels.push(num);
                input = new_input;
                loop {
                    let (c, new_input) = next_char(input).map_err(|e| de::Error::custom(e))?;
                    input = new_input;
                    if c == ' ' {
                        let (num, new_input) =
                            parse_unsigned(input).map_err(|e| de::Error::custom(e))?;
                        input = new_input;
                        levels.push(num);
                    } else {
                        break;
                    }
                }
                reports.push(Report { levels });
            } else {
                break;
            }
        }

        Ok(Day2Input { reports })
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

fn level_safety(level: &[u32]) -> bool {
    if level.is_empty() || level.len() == 1 {
        return true;
    }
    let positive = level[0] < level[1];
    level.windows(2).all(|w| {
        if positive {
            (1..4).contains(&w[1].saturating_sub(w[0]))
        } else {
            (1..4).contains(&w[0].saturating_sub(w[1]))
        }
    })
}

pub fn calculate(input: &str) -> anyhow::Result<(i32, u64)> {
    let input: Day2Input = deserialize(input)?;

    let mut safe_reports = 0;
    for report in &input.reports {
        if level_safety(&report.levels) {
            safe_reports += 1;
        }
    }
    let mut safe_reports_p2 = 0;
    for report in &input.reports {
        let mut passed = false;
        for i in 0..report.levels.len() {
            let mut levels_copy = report.levels.clone();
            levels_copy.remove(i);
            if level_safety(&levels_copy) {
                passed = true;
            }
        }
        if passed {
            safe_reports_p2 += 1;
        }
    }
    Ok((safe_reports, safe_reports_p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_data() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!(calculate(input).unwrap(), (2, 4));
    }

    #[test]
    fn invalid_short_inc() {
        let input = "1 7\n";
        assert_eq!(calculate(input).unwrap(), (0, 1));
    }

    #[test]
    fn invalid_short_dec() {
        let input = "7 1\n";
        assert_eq!(calculate(input).unwrap(), (0, 1));
    }

    #[test]
    fn invalid_short_flip() {
        let input = "7 1 2\n";
        assert_eq!(calculate(input).unwrap(), (0, 1));
    }

    #[test]
    fn invalid_short_flip_large_drop() {
        let input = "7 1 8\n";
        assert_eq!(calculate(input).unwrap(), (0, 1));
    }


    #[test]
    fn invalid_last_inc() {
        let input = "1 2 3 9\n";
        assert_eq!(calculate(input).unwrap(), (0, 1));
    }

    #[test]
    fn invalid_first_inc() {
        let input = "1 7 8 9\n";
        assert_eq!(calculate(input).unwrap(), (0, 1));
    }
}