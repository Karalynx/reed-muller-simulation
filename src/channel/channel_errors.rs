
use std::fmt;

use crate::math::vector::BinaryVector;

/// Contains indexes of bits that were corrupted during transmission.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ChannelErrors(Box::<[usize]>);

impl ChannelErrors {
    /// Compares the bytes of two binary strings. Whitespaces are ignored.
    /// 
    /// Returns an array of error indexes.
    #[must_use]
    pub fn from_bytes<S: AsRef<[u8]>>(str1: S, str2: S) -> Self {
        let (str1, str2) = (
            str1.as_ref()
                .iter()
                .filter(|c| !c.is_ascii_whitespace())
                .collect::<Vec<&u8>>(),
            str2.as_ref()
                .iter()
                .filter(|c| !c.is_ascii_whitespace())
                .collect::<Vec<&u8>>(),
        );
    
        let (min, max) = 
        if str1.len() > str2.len() {
            (str2.len(), str1.len())
        } 
        else {
            (str1.len(), str2.len())
        };
    
        let mut errors = Vec::with_capacity(max);
        for idx in 0 .. min {
            if str1[idx] != str2[idx] {
                errors.push(idx);
            }
        }
    
        // Every bit that is not present is an error
        for idx in min .. max {
            errors.push(idx);
        }
        
        Self(errors.into_boxed_slice())
    }

    /// Compares two binary vectors.
    /// 
    /// Returns an array of error indexes
    pub fn from_vectors(vec1: &BinaryVector, vec2: &BinaryVector) -> Self {

        let (min, max) = 
        if vec1.cols() > vec2.cols() {
            (vec2.cols() as usize, vec1.cols() as usize)
        } 
        else {
            (vec1.cols() as usize, vec2.cols() as usize)
        };
    
        let mut errors = Vec::with_capacity(max);
        for idx in 0 .. min {
            if vec1.inner()[idx] != vec2.inner()[idx] {
                errors.push(idx);
            }
        }
    
        // Every bit that is not present is an error
        for idx in min .. max {
            errors.push(idx);
        }

        Self(errors.into_boxed_slice())
    }

    /// Returns a reference to error indexes.
    #[inline]
    pub fn get(&self) -> &[usize] {
        &self.0
    }
}

impl fmt::Display for ChannelErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_word = match self.0.len() {
            x if ((x % 100 > 10 && x % 100 < 20) || x % 10 == 0) => "klaidų",
            x if (x % 10 == 1) => "klaida",
            _ => "klaidos"
        };

        write!(f, "{} {error_word}: {:?}", self.0.len(), self.0)
    }
}

impl From::<Vec<usize>> for ChannelErrors {
    fn from(value: Vec<usize>) -> Self {
        Self(value.into_boxed_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn errors_from_strings() {
        assert_eq!(
            ChannelErrors::from_bytes("", ""),
            ChannelErrors(Vec::new().into())
        );

        assert_eq!(
            ChannelErrors::from_bytes("1", ""),
            ChannelErrors(vec![0].into())
        );

        assert_eq!(
            ChannelErrors::from_bytes("", "1"),
            ChannelErrors(vec![0].into())
        );

        assert_eq!(
            ChannelErrors::from_bytes("0", "1"),
            ChannelErrors(vec![0].into())
        );

        assert_eq!(
            ChannelErrors::from_bytes("1", "1"),
            ChannelErrors(vec![].into())
        );

        assert_eq!(
            ChannelErrors::from_bytes("10  10", "10    10"),
            ChannelErrors(vec![].into())
        );

        assert_eq!(
            ChannelErrors::from_bytes("10  101", "10    10"),
            ChannelErrors(vec![4].into())
        );

        assert_eq!(
            ChannelErrors::from_bytes("10  101", "10"),
            ChannelErrors(vec![2, 3, 4].into())
        );
    }

    #[test]
    pub fn errors_from_vectors() {
        assert_eq!(
            ChannelErrors::from_vectors(
                &BinaryVector::from_binary_bytes("0").unwrap(), 
                &BinaryVector::from_binary_bytes("1").unwrap()
            ),
            ChannelErrors(vec![0].into())
        );

        assert_eq!(
            ChannelErrors::from_vectors(
                &BinaryVector::from_binary_bytes("1").unwrap(), 
                &BinaryVector::from_binary_bytes("1").unwrap()
            ),
            ChannelErrors(vec![].into())
        );

        assert_eq!(
            ChannelErrors::from_vectors(
                &BinaryVector::from_binary_bytes("101010").unwrap(), 
                &BinaryVector::from_binary_bytes("101010").unwrap()
            ),
            ChannelErrors(vec![].into())
        );

        assert_eq!(
            ChannelErrors::from_vectors(
                &BinaryVector::from_binary_bytes("10101").unwrap(), 
                &BinaryVector::from_binary_bytes("1010").unwrap()
            ),
            ChannelErrors(vec![4].into())
        );

        assert_eq!(
            ChannelErrors::from_vectors(
                &BinaryVector::from_binary_bytes("10101").unwrap(), 
                &BinaryVector::from_binary_bytes("10").unwrap()
            ),
            ChannelErrors(vec![2, 3, 4].into())
        );
    }
}