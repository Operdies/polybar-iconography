use std::fmt::Display;

use thiserror::Error;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Color {
    pub a: Option<u8>,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Error, Debug, PartialEq)]
pub enum ColorParseError {
    #[error("String of length {0} is not a valid color. Only 3|4|6|8 are valid lengths.")]
    InvalidStringLength(usize),
    #[error("Symbol {0} is out of range. 0-9a-fA-F are valid color ranges.")]
    CharOutOfRange(char),
}
impl TryFrom<String> for Color {
    type Error = ColorParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Color::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.chars().collect::<Vec<_>>();
        let mut slice = &value[..];
        if value.first().is_some_and(|v| *v == '#') {
            slice = &value[1..];
        }
        // only 0-9a-fA-F are valid symbols
        // only 3|4|6|8 are valid string lengths (after removing the leading #)

        for ch in slice.iter() {
            match ch {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {}
                x => return Err(ColorParseError::CharOutOfRange(*x)),
            }
        }

        let expanded = if slice.len() < 6 {
            let mut chars = vec![];
            for ch in slice {
                chars.push(*ch);
                chars.push(*ch);
            }
            chars
        } else {
            slice.to_vec()
        };

        let mut col = Color::default();
        match expanded.len() {
            6 => {
                col.r = u8::from_str_radix(&expanded[0..2].iter().collect::<String>(), 16).unwrap();
                col.g = u8::from_str_radix(&expanded[2..4].iter().collect::<String>(), 16).unwrap();
                col.b = u8::from_str_radix(&expanded[4..6].iter().collect::<String>(), 16).unwrap();
            }
            8 => {
                col.a = Some(
                    u8::from_str_radix(&expanded[0..2].iter().collect::<String>(), 16).unwrap(),
                );
                col.r = u8::from_str_radix(&expanded[2..4].iter().collect::<String>(), 16).unwrap();
                col.g = u8::from_str_radix(&expanded[4..6].iter().collect::<String>(), 16).unwrap();
                col.b = u8::from_str_radix(&expanded[6..8].iter().collect::<String>(), 16).unwrap();
            }
            _ => return Err(ColorParseError::InvalidStringLength(slice.len())),
        }

        Ok(col)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self;
        let s = if let Some(alpha) = value.a {
            format!(
                "#{:02X}{:02X}{:02X}{:02X}",
                alpha, value.r, value.g, value.b
            )
        } else {
            format!("#{:02X}{:02X}{:02X}", value.r, value.g, value.b)
        };
        f.write_str(&s)
    }
}

#[test]
fn test_colors() {
    assert_eq!(
        Color::try_from("#12345").unwrap_err(),
        ColorParseError::InvalidStringLength(5)
    );
    assert_eq!(
        Color::try_from("12345").unwrap_err(),
        ColorParseError::InvalidStringLength(5)
    );
    assert_eq!(
        Color::try_from("#123").unwrap(),
        Color {
            a: None,
            r: 0x11,
            g: 0x22,
            b: 0x33
        }
    );
    assert_eq!(
        Color::try_from("#12abcdef").unwrap(),
        Color {
            a: Some(0x12),
            r: 0xab,
            g: 0xcd,
            b: 0xef
        }
    );
    assert_eq!(
        Color::try_from("12abcdef").unwrap(),
        Color {
            a: Some(0x12),
            r: 0xab,
            g: 0xcd,
            b: 0xef
        }
    );
    assert_eq!(
        Color::try_from("abcdef").unwrap(),
        Color {
            a: None,
            r: 0xab,
            g: 0xcd,
            b: 0xef
        }
    );
    assert_eq!(
        Color::try_from("123").unwrap(),
        Color {
            a: None,
            r: 0x11,
            g: 0x22,
            b: 0x33
        }
    );
}
