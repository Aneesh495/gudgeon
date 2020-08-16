//! LOLCODE runtime value system
//! Implements dynamically-typed values conforming to the LOLCODE 1.2 specification:
//! NOOB (nil), TROOF (boolean), NUMBR (integer), NUMBAR (float), YARN (string),
//! plus high-performance BUKET (array) and KITTEN (table/dict) extensions.

use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum LolValue {
    Noob,
    Troof(bool),
    Numbr(i64),
    Numbar(f64),
    Yarn(String),
    Buket(Vec<LolValue>),
    Kitten(HashMap<String, LolValue>),
}

impl LolValue {
    pub fn is_noob(&self) -> bool {
        matches!(self, LolValue::Noob)
    }

    pub fn to_troof(&self) -> bool {
        match self {
            LolValue::Noob => false,
            LolValue::Troof(b) => *b,
            LolValue::Numbr(n) => *n != 0,
            LolValue::Numbar(f) => *f != 0.0 && !f.is_nan(),
            LolValue::Yarn(s) => !s.is_empty(),
            LolValue::Buket(arr) => !arr.is_empty(),
            LolValue::Kitten(map) => !map.is_empty(),
        }
    }

    pub fn to_numbr(&self) -> i64 {
        match self {
            LolValue::Noob => 0,
            LolValue::Troof(b) => if *b { 1 } else { 0 },
            LolValue::Numbr(n) => *n,
            LolValue::Numbar(f) => *f as i64,
            LolValue::Yarn(s) => s.trim().parse::<i64>().unwrap_or(0),
            LolValue::Buket(arr) => arr.len() as i64,
            LolValue::Kitten(map) => map.len() as i64,
        }
    }

    pub fn to_numbar(&self) -> f64 {
        match self {
            LolValue::Noob => 0.0,
            LolValue::Troof(b) => if *b { 1.0 } else { 0.0 },
            LolValue::Numbr(n) => *n as f64,
            LolValue::Numbar(f) => *f,
            LolValue::Yarn(s) => s.trim().parse::<f64>().unwrap_or(0.0),
            LolValue::Buket(arr) => arr.len() as f64,
            LolValue::Kitten(map) => map.len() as f64,
        }
    }

    pub fn to_yarn(&self) -> String {
        match self {
            LolValue::Noob => "NOOB".to_string(),
            LolValue::Troof(b) => if *b { "WIN".to_string() } else { "FAIL".to_string() },
            LolValue::Numbr(n) => n.to_string(),
            LolValue::Numbar(f) => {
                if f.fract() == 0.0 && !f.is_infinite() && !f.is_nan() && f.abs() < 1e15 {
                    format!("{:.1}", f)
                } else {
                    format!("{}", f)
                }
            }
            LolValue::Yarn(s) => s.clone(),
            LolValue::Buket(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_yarn()).collect();
                format!("[{}]", items.join(", "))
            }
            LolValue::Kitten(map) => {
                let mut entries: Vec<String> = map.iter().map(|(k, v)| format!("{}: {}", k, v.to_yarn())).collect();
                entries.sort();
                format!("{{{}}}", entries.join(", "))
            }
        }
    }

    pub fn add(&self, other: &LolValue) -> LolValue {
        match (self, other) {
            (LolValue::Numbr(a), LolValue::Numbr(b)) => LolValue::Numbr(a.wrapping_add(*b)),
            _ => LolValue::Numbar(self.to_numbar() + other.to_numbar()),
        }
    }

    pub fn sub(&self, other: &LolValue) -> LolValue {
        match (self, other) {
            (LolValue::Numbr(a), LolValue::Numbr(b)) => LolValue::Numbr(a.wrapping_sub(*b)),
            _ => LolValue::Numbar(self.to_numbar() - other.to_numbar()),
        }
    }

    pub fn mul(&self, other: &LolValue) -> LolValue {
        match (self, other) {
            (LolValue::Numbr(a), LolValue::Numbr(b)) => LolValue::Numbr(a.wrapping_mul(*b)),
            _ => LolValue::Numbar(self.to_numbar() * other.to_numbar()),
        }
    }

    pub fn div(&self, other: &LolValue) -> LolValue {
        let b = other.to_numbar();
        if b == 0.0 {
            LolValue::Numbar(f64::NAN)
        } else {
            LolValue::Numbar(self.to_numbar() / b)
        }
    }

    pub fn modulo(&self, other: &LolValue) -> LolValue {
        match (self, other) {
            (LolValue::Numbr(a), LolValue::Numbr(b)) => {
                if *b == 0 {
                    LolValue::Numbr(0)
                } else {
                    LolValue::Numbr(a % b)
                }
            }
            _ => {
                let b = other.to_numbar();
                if b == 0.0 {
                    LolValue::Numbar(f64::NAN)
                } else {
                    LolValue::Numbar(self.to_numbar() % b)
                }
            }
        }
    }

    pub fn max(&self, other: &LolValue) -> LolValue {
        match (self, other) {
            (LolValue::Numbr(a), LolValue::Numbr(b)) => LolValue::Numbr((*a).max(*b)),
            _ => LolValue::Numbar(self.to_numbar().max(other.to_numbar())),
        }
    }

    pub fn min(&self, other: &LolValue) -> LolValue {
        match (self, other) {
            (LolValue::Numbr(a), LolValue::Numbr(b)) => LolValue::Numbr((*a).min(*b)),
            _ => LolValue::Numbar(self.to_numbar().min(other.to_numbar())),
        }
    }

    pub fn equal_to(&self, other: &LolValue) -> bool {
        match (self, other) {
            (LolValue::Noob, LolValue::Noob) => true,
            (LolValue::Troof(a), LolValue::Troof(b)) => a == b,
            (LolValue::Numbr(a), LolValue::Numbr(b)) => a == b,
            (LolValue::Numbar(a), LolValue::Numbar(b)) => (a - b).abs() < 1e-9 || (a.is_nan() && b.is_nan()),
            (LolValue::Numbr(a), LolValue::Numbar(b)) | (LolValue::Numbar(b), LolValue::Numbr(a)) => {
                ((*a as f64) - b).abs() < 1e-9
            }
            (LolValue::Yarn(a), LolValue::Yarn(b)) => a == b,
            _ => false,
        }
    }
}

impl fmt::Display for LolValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_yarn())
    }
}
