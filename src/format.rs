use std::{
    error::Error,
    fmt::{Debug, Display},
    ops::RangeInclusive,
};

use crate::{CronExpr, Field, ListValue, StepValue};

impl CronExpr {
    pub fn parse<T: AsRef<str>>(expr_str: T) -> ParseResult<Self> {
        let expr_str = expr_str.as_ref();

        if expr_str.is_empty() {
            return Err(ParseError::Empty);
        }

        let mut expr_fields = expr_str.split(|c: char| c.is_ascii_whitespace());

        let second = parse_field(expr_fields.next().ok_or(ParseError::Incomplete)?)?;
        let minute = parse_field(expr_fields.next().ok_or(ParseError::Incomplete)?)?;
        let hour = parse_field(expr_fields.next().ok_or(ParseError::Incomplete)?)?;
        let day = parse_field(expr_fields.next().ok_or(ParseError::Incomplete)?)?;
        let month = parse_field(expr_fields.next().ok_or(ParseError::Incomplete)?)?;
        let day_of_week = parse_field(expr_fields.next().ok_or(ParseError::Incomplete)?)?;

        Ok(Self {
            second,
            minute,
            hour,
            day,
            month,
            day_of_week,
        })
    }
}

impl ToString for CronExpr {
    fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            write_field(&self.second),
            write_field(&self.minute),
            write_field(&self.hour),
            write_field(&self.day),
            write_field(&self.month),
            write_field(&self.day_of_week)
        )
    }
}

#[derive(Debug)]
pub enum ParseError {
    Empty,
    Field,
    Incomplete,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <ParseError as Debug>::fmt(&self, f)
    }
}

impl Error for ParseError {}

pub type ParseResult<T> = core::result::Result<T, ParseError>;

fn parse_field(field: &str) -> ParseResult<Field> {
    if field == "*" {
        return Ok(Field::All);
    }

    if let Ok(int_val) = field.parse::<u8>() {
        return Ok(Field::Value(int_val));
    }

    if let Some(step) = parse_step(field) {
        return Ok(step);
    }

    if let Some(list) = parse_list(field) {
        return Ok(Field::List(list));
    }

    if let Some(range) = parse_range(field) {
        return Ok(Field::Range(range));
    }

    Err(ParseError::Field)
}

fn parse_step(field: &str) -> Option<Field> {
    let (range, step) = field.rsplit_once('/')?;

    let s: u8 = step.parse().ok()?;

    let r = if range == "*" {
        StepValue::All
    } else {
        StepValue::Range(parse_range(range)?)
    };

    Some(Field::Step(r, s))
}

fn parse_range(field: &str) -> Option<RangeInclusive<u8>> {
    let (start, end) = field.split_once('-')?;

    let s: u8 = start.parse().ok()?;
    let e: u8 = end.parse().ok()?;

    Some(s..=e)
}

fn parse_list(field: &str) -> Option<Vec<ListValue>> {
    let mut list = Vec::new();

    for item in field.split(',') {
        let item_value = if let Ok(int_val) = item.parse::<u8>() {
            ListValue::Value(int_val)
        } else {
            ListValue::Range(parse_range(item)?)
        };

        list.push(item_value);
    }

    if list.len() < 2 {
        return None;
    }

    Some(list)
}

fn write_field(field: &Field) -> String {
    match field {
        Field::All => String::from("*"),
        Field::Value(v) => v.to_string(),
        Field::Range(r) => write_range(r),
        Field::List(l) => l
            .iter()
            .map(write_list_value)
            .collect::<Vec<String>>()
            .join(","),
        Field::Step(sv, s) => write_step_value(sv, *s),
    }
}

fn write_list_value(list_val: &ListValue) -> String {
    match list_val {
        ListValue::Value(v) => v.to_string(),
        ListValue::Range(r) => write_range(r),
    }
}

fn write_range(r: &RangeInclusive<u8>) -> String {
    format!("{}-{}", r.start(), r.end())
}

fn write_step_value(sv: &StepValue, s: u8) -> String {
    let sv = match sv {
        StepValue::All => String::from("*"),
        StepValue::Range(r) => write_range(r),
    };

    format!("{}/{}", sv, s)
}

#[cfg(feature = "serde")]
mod serde {
    use crate::CronExpr;
    use serde::{Deserialize, Serialize};

    impl Serialize for CronExpr {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de> Deserialize<'de> for CronExpr {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let expr = String::deserialize(deserializer)?;
            Ok(CronExpr::parse(&expr).map_err(|e| serde::de::Error::custom(e.to_string()))?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CronExpr, Field, ListValue, StepValue};

    #[test]
    fn parse_expr() {
        let expr = CronExpr::parse("30 0-30/5 13-15,18\t* * 1-5").unwrap();
        assert_eq!(
            CronExpr {
                second: Field::Value(30),
                minute: Field::Step(StepValue::Range(0..=30), 5),
                hour: Field::List(vec![ListValue::Range(13..=15), ListValue::Value(18)]),
                day_of_week: Field::Range(1..=5),
                ..Default::default()
            },
            expr
        );
    }

    #[test]
    fn expr_to_string() {
        let expr = CronExpr {
            second: Field::Value(30),
            minute: Field::Step(StepValue::Range(0..=30), 5),
            hour: Field::List(vec![ListValue::Range(13..=15), ListValue::Value(18)]),
            day_of_week: Field::Range(1..=5),
            ..Default::default()
        }
        .to_string();

        assert_eq!("30 0-30/5 13-15,18 * * 1-5", expr);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde() {
        use serde_test::{assert_tokens, Token};

        assert_tokens(
            &CronExpr::parse("30 0-30/5 13-15,18 * * 1-5").unwrap(),
            &[Token::Str("30 0-30/5 13-15,18 * * 1-5")],
        );
    }
}
