use core::fmt;

use crate::{AbsoluteOid, Arc, Arc0, Arc1, RelativeOid, RootOid};

impl fmt::Display for AbsoluteOid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.root(), self.tail())
    }
}

impl fmt::Debug for AbsoluteOid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for RootOid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (arc0, arc1) = self.into_arcs();
        write!(f, "{}.{}", arc0 as u8, arc1.as_u8())
    }
}

impl fmt::Debug for RootOid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for RelativeOid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for arc in self.arcs() {
            write!(f, ".{}", arc)?;
        }
        Ok(())
    }
}

impl fmt::Debug for RelativeOid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub struct StrArcs<'a> {
    parts: core::str::Split<'a, char>,
}

impl<'a> StrArcs<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            parts: s.split('.'),
        }
    }
}

impl core::iter::Iterator for StrArcs<'_> {
    type Item = Result<Arc, OidParsingError>;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.parts.next()?;
        if value.is_empty() {
            return None;
        }
        Some(value.parse().map_err(|_| OidParsingError::OverflowError))
    }
}

pub fn parse_absolute(s: &str) -> Result<(RootOid, StrArcs), OidParsingError> {
    check_not_empty(s)?;
    check_str(s)?;

    let s = strip_leading_dot(s);
    let mut parts = s.split('.');

    // must have at least on segment if `check_not_empty` has passed
    let arc0_str = parts.next().unwrap();
    let arc0 = arc0_str
        .parse()
        .map_err(|_| OidParsingError::OverflowError)?;
    let arc0 = Arc0::new(arc0).map_err(|_| OidParsingError::OverflowError)?;
    let arc1_str = parts.next().ok_or(OidParsingError::NoArc1)?;
    let arc1 = arc1_str
        .parse()
        .map_err(|_| OidParsingError::OverflowError)?;
    let arc1 = Arc1::new(arc1).map_err(|_| OidParsingError::OverflowError)?;

    let root = RootOid::new(arc0, arc1);
    let arc = StrArcs { parts };

    Ok((root, arc))
}

pub fn parse_relative(s: &str) -> Result<StrArcs, OidParsingError> {
    check_str(s)?;

    let s = strip_leading_dot(s);
    Ok(StrArcs::new(s))
}
#[derive(Debug, Clone)]
pub enum OidParsingError {
    InvalidChar(usize),
    IntegerExpected(usize),
    OverflowError,
    NoArc1,
}

fn check_not_empty(s: &str) -> Result<(), OidParsingError> {
    if s.is_empty() || s == "." {
        return Err(OidParsingError::IntegerExpected(0));
    }
    Ok(())
}

fn check_str(s: &str) -> Result<(), OidParsingError> {
    if let Some(p) = s.chars().position(|c| !c.is_digit(10) && c != '.') {
        return Err(OidParsingError::InvalidChar(p));
    }
    if let Some(p) = s.find("..") {
        return Err(OidParsingError::IntegerExpected(p));
    }
    if s.len() > 1 && s.as_bytes()[s.len() - 1] == b'.' {
        return Err(OidParsingError::IntegerExpected(s.len() - 1));
    }
    Ok(())
}

fn strip_leading_dot(s: &str) -> &str {
    if s.as_bytes().get(0) == Some(&b'.') {
        &s[1..]
    } else {
        s
    }
}

#[cfg(feature = "alloc")]
mod owned {
    use core::{fmt, ops::Deref, str::FromStr};

    use crate::{AbsoluteOidVec, RelativeOidVec};

    use super::{parse_absolute, parse_relative, OidParsingError};

    impl FromStr for AbsoluteOidVec {
        type Err = OidParsingError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (root, arcs) = parse_absolute(s)?;
            let mut vec = AbsoluteOidVec::from_root(root);
            for arc in arcs {
                vec.push(arc?);
            }

            Ok(vec)
        }
    }

    impl FromStr for RelativeOidVec {
        type Err = OidParsingError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let arcs = parse_relative(s)?;
            let mut vec = RelativeOidVec::default();
            for arc in arcs {
                vec.push(arc?);
            }

            Ok(vec)
        }
    }

    impl fmt::Display for AbsoluteOidVec {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(self.deref(), f)
        }
    }

    impl fmt::Debug for AbsoluteOidVec {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(self.deref(), f)
        }
    }


    impl fmt::Display for RelativeOidVec {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(self.deref(), f)
        }
    }

    impl fmt::Debug for RelativeOidVec {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(self.deref(), f)
        }
    }
}
