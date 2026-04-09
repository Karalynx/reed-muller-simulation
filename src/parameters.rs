
use std::{error::Error, fmt, num::{IntErrorKind, NonZeroU8}, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseProbabilityError {
    kind: ProbErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbErrorKind {
    Invalid,
    Range,
}

impl ParseProbabilityError {
    /// Outputs the detailed cause of parsing a probability failing.
    pub const fn kind(&self) -> &ProbErrorKind {
        &self.kind
    }
}

impl Error for ParseProbabilityError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match self.kind {
            ProbErrorKind::Invalid => "invalid probability literal",
            ProbErrorKind::Range => "probability outside of [0; 1] range",
        }
    }
}

impl fmt::Display for ParseProbabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(deprecated)]
        self.description().fmt(f)
    }
}

/// A wrapper around `f32` that ensures the value is within range of \[0-1].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Probability(f32);

impl Probability {
    /// Creates a `Probability` from `val`.
    /// 
    /// Returns `None` if `val` is outside of \[0-1] range.
    #[must_use]
    #[inline]
    pub fn new(val: f32) -> Option<Self> {
        if val < 0.0 || val > 1.0 {
            return None;
        }
        return Some(Self(val))
    }

    /// Creates a `Probability` from `val` without checking for validity.
    #[inline]
    pub unsafe fn new_unchecked(val: f32) -> Self {
        debug_assert!(val >= 0.0 && val <= 1.0);
        return Self(val)
    }

    /// Returns the underlying value inside the `Probability`.
    #[inline]
    pub const fn get(&self) -> f32 {
        self.0
    }
}

impl ToString for Probability {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for Probability {
    type Err = ParseProbabilityError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        // Ensure parsing works for Lithuanian locale
        let src = src.replace(',', ".");
        
        match src.parse::<f32>() {
            Ok(x) => {
                if x < 0.0 || x > 1.0 {
                    Err(ParseProbabilityError { kind: ProbErrorKind::Range })
                }
                else {
                    Ok(Self(x))
                }
            },
            Err(_) => Err(
                ParseProbabilityError{ kind: ProbErrorKind::Invalid }
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseMullerError {
    kind: MullerErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MullerErrorKind {
    Invalid,
    Range,
}

impl ParseMullerError {
    /// Outputs the detailed cause of parsing an integer failing.
    pub const fn kind(&self) -> &MullerErrorKind {
        &self.kind
    }
}

impl Error for ParseMullerError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match self.kind {
            MullerErrorKind::Invalid => "invalid muller parameter literal",
            MullerErrorKind::Range => "muller parameter outside of [1; 10] range",
        }
    }
}

impl fmt::Display for ParseMullerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(deprecated)]
        self.description().fmt(f)
    }
}

/// A wrapper around `NonZeroU8` that ensures the value is within range of \[1-10].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Muller(NonZeroU8);

impl Muller {
    /// Creates a `Muller` parameter from `val`.
    /// 
    /// Returns `None` if `val` is outside of \[1-10] range.
    pub const fn new(val: u8) -> Option<Self> {
        if val > 10 || val == 0 {
            None
        }
        else {
            unsafe { Some(Self(NonZeroU8::new_unchecked(val))) }
        }
    }

    /// Creates a `Muller` parameter from `val` without checking for validity.
    #[inline]
    pub const unsafe fn new_unchecked(val: u8) -> Self {
        debug_assert!(val != 0 && val <= 10);
        Self(NonZeroU8::new_unchecked(val))
    }

    /// Returns the underlying `u8` value inside the `Muller` parameter.
    #[inline]
    pub const fn get(&self) -> u8 {
        self.0.get()
    }

    /// Returns the underlying `NonZeroU8` value inside the `Muller` parameter.
    #[inline]
    pub const fn get_non_zero(&self) -> NonZeroU8 {
        self.0
    }

    /// Returns the amount of rows a generation matrix with parameter `m` should have.
    #[inline]
    pub const fn rows(&self) -> u32 {
        self.0.get() as u32 + 1
    }

    /// Returns the amount of columns a generation matrix with parameter `m` should have.
    #[inline]
    pub const fn cols(&self) -> u32 {
        2u32 << (self.0.get() - 1)
    }
}

impl ToString for Muller {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for Muller {
    type Err = ParseMullerError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src.parse::<NonZeroU8>() {
            Ok(x) => {
                if x.get() > u16::BITS as u8 - 1 {
                    Err(ParseMullerError{kind: MullerErrorKind::Range})
                }
                else {
                    Ok(Self(x))
                }
            },
            Err(err) => {
                Err(match err.kind() {
                    IntErrorKind::Empty => ParseMullerError{kind: MullerErrorKind::Invalid},
                    IntErrorKind::InvalidDigit => ParseMullerError{kind: MullerErrorKind::Invalid},
                    IntErrorKind::PosOverflow => ParseMullerError{kind: MullerErrorKind::Range},
                    IntErrorKind::NegOverflow => ParseMullerError{kind: MullerErrorKind::Range},
                    IntErrorKind::Zero => ParseMullerError{kind: MullerErrorKind::Range},
                    _ => ParseMullerError{kind: MullerErrorKind::Invalid},
                })
            }
        }
    }
}

/// Scenarios for showcasing `Reed-Muller` code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scenario {
    SendingVector,
    SendingText,
    SendingImage
}

/// Parameters used while showcasing `Reed-Muller` code.
/// - `m` is used to create a generation matrix.
/// - `p` is the probability for an error to occur for each bit.
/// - `scenario` is the chosen testing scenario.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Parameters {
    pub m: Muller,
    pub p: Probability,
    pub scenario: Scenario
}

impl Default for Parameters {
    fn default() -> Self {
        unsafe { 
            Self { 
                m: Muller::new_unchecked(3),
                p: Probability::new_unchecked(0.1), 
                scenario: Scenario::SendingVector
            }
        }
    }
}

impl Parameters {
    /// Creates `Parameters` with default values.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probability_new() {
        assert_eq!(Probability::new(1.0), Some(Probability(1.0)));

        assert_eq!(Probability::new(0.0), Some(Probability(0.0)));

        assert_eq!(Probability::new(1.01), None);

        assert_eq!(Probability::new(-0.01), None);
    }

    #[test]
    fn muller_new() {
        assert_eq!(Muller::new(1), Some(Muller(1.try_into().unwrap())));

        assert_eq!(Muller::new(10), Some(Muller(10.try_into().unwrap())));

        assert_eq!(Muller::new(0), None);
        
        assert_eq!(Muller::new(11), None);
    }
}