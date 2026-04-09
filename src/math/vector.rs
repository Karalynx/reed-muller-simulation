
use super::matrix::Matrix;

/// A one-dimensional matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Vector(pub(super) Matrix);

impl Vector {

    /// Calculates the dot product between `self` and `Matrix`.
    #[inline]
    fn dot_product_inner(&self, other: &Matrix) -> Self {
        let self_cols = self.cols() as usize;
        let other_cols = other.cols() as usize;

        let mut data = vec![0; other_cols].into_boxed_slice();
        for col in 0 .. other_cols {
            let value = &mut data[col];
            for k in 0 .. self_cols {
                *value += self.inner()[k] * other.inner()[(k * other_cols) + col];
            }
        }

        Self::from(data)
    }

    /// Multiplies a vector and a matrix.
    ///
    /// Returns `None` if the column count of `self` doesn't match the row count of `other`.
    pub fn dot_product<M: AsRef<Matrix>>(&self, other: M) -> Option<Self> {
        let other = other.as_ref();
        
        if self.cols() != other.rows() {
            return None;
        }

        Some(self.dot_product_inner(other))
    }

    /// Multiplies a vector and a matrix without checking for dimension validity.
    /// 
    /// Returns a new [`Vector`].
    pub unsafe fn dot_product_unchecked<M: AsRef<Matrix>>(&self, other: M) -> Self {
        debug_assert!(self.cols() == other.as_ref().rows());
        self.dot_product_inner(other.as_ref())
    }

    /// Returns the number of rows `self` has.
    /// 
    /// Always returns 1.
    #[inline]
    pub const fn rows(&self) -> u32 {
        1
    }

    /// Returns the number of columns `self` has.
    #[inline]
    pub const fn cols(&self) -> u32 {
        self.0.cols()
    }

    /// Returns a reference to inner data of `self`.
    #[inline]
    pub const fn inner(&self) -> &[i32] {
        &self.0.inner()
    }

    /// Returns a reference to inner data of `self`.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut [i32] {
        self.0.inner_mut()
    }

    /// Returns the matrix representing `self`.
    #[inline]
    pub const fn matrix(&self) -> &Matrix {
        &self.0
    }
}

impl From<Vec<i32>> for Vector {
    #[inline]
    fn from(val: Vec<i32>) -> Self {
        Self(Matrix::from(val))
    }
}

impl From<Box<[i32]>> for Vector {
    #[inline]
    fn from(val: Box<[i32]>) -> Self {
        Self(Matrix::from(val))
    }
}

impl From<BinaryVector> for Vector {
    #[inline]
    fn from(val: BinaryVector) -> Self {
        val.0
    }
}

impl AsRef<Vector> for Vector {
    #[inline]
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl AsRef<Matrix> for Vector {
    #[inline]
    fn as_ref(&self) -> &Matrix {
        &self.0
    }
}

impl AsRef<[i32]> for Vector {
    #[inline]
    fn as_ref(&self) -> &[i32] {
        &self.inner()
    }
}

/// A one-dimensional matrix that can only have 0s and 1s as values.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct BinaryVector(Vector);

impl BinaryVector {
    /// Creates a new binary vector from a `src` that contains only 0s and 1s. Whitespaces are skipped.
    ///
    /// Returns `None` if a non-binary character was detected or the string is empty.
    pub fn from_binary_bytes<S: AsRef<[u8]>>(src: S) -> Option<Self> {
        let src = src.as_ref();
        let mut data = vec![0; src.len()];

        let mut whtspc_offset = 0;
        for (idx, ch) in src.iter().enumerate() {
            match ch {
                b'0' => {},
                b'1' => data[idx - whtspc_offset] = 1,
                x if x.is_ascii_whitespace() => { data.pop(); whtspc_offset += 1; },
                _ => return None
            }
        }

        match data.len() {
            0 => None,
            _ => Some(Self(Vector::from(data)))
        }
    }

    /// Creates a new binary vector from `vec` without checking for validity.
    pub unsafe fn from_vec_unchecked(vec: Vec<i32>) -> Self {
        Self(Vector::from(vec))
    }

    /// Creates a new binary vector by extracting each bit of `src`.
    pub fn from_bits<S: AsRef<[u8]>>(src: S) -> Option<Self> {
        const BYTE_SIZE: usize = u8::BITS as usize;
        let msg_bytes = src.as_ref();
        if msg_bytes.is_empty() {
            return None;
        }
    
        let len_bits = msg_bytes.len() * BYTE_SIZE;
        let mut bits = vec![0; len_bits].into_boxed_slice();

        for i in 0 .. len_bits {
            let byte = msg_bytes[i >> 3];
            // Extract bit from the left side
            bits[i] = (byte >> ((BYTE_SIZE - 1) - (i & (BYTE_SIZE - 1))) & 1) as i32;
        }

        Some(Self(Vector::from(bits)))
    }

    /// Calculates the dot product between `self` and `Matrix` with modulo of 2.
    #[inline]
    fn dot_product_inner(&self, other: &Matrix) -> Self {
        let self_cols = self.cols() as usize;
        let other_cols = other.cols() as usize;

        let mut data = vec![0; other_cols].into_boxed_slice();
        for col in 0 .. other_cols {
            let value = &mut data[col];
            for k in 0 .. self_cols {
                *value += self.inner()[k] * other.inner()[(k * other_cols) + col];
            }
            *value %= 2;
        }

        Self(Vector::from(data))
    }

    /// Multiplies a binary vector and a matrix with modulo of 2.
    ///
    /// Returns `None` if the column count of `self` doesn't match the row count of `other`.
    pub fn dot_product<M: AsRef<Matrix>>(&self, other: M) -> Option<Self> {
        let other = other.as_ref();
        
        if self.cols() != other.rows() {
            return None;
        }

        Some(self.dot_product_inner(other))
    }

    /// Multiplies a binary vector and a matrix with modulo of 2 without checking for dimension validity.
    /// 
    /// Returns a new [`BinaryVector`].
    pub unsafe fn dot_product_unchecked<M: AsRef<Matrix>>(&self, other: M) -> Self {
        debug_assert!(self.cols() == other.as_ref().rows());
        self.dot_product_inner(other.as_ref())
    }

    /// Substitutes all zeroes in a binary vector with -1s.
    /// 
    /// Returns a new vector.
    pub fn substitute_zeroes(&self) -> Vector {
        let mut data = self.0.clone();
        for num in data.inner_mut() {
            if *num == 0 {
                *num = -1;
            }
        }
        
        data
    }

    /// Returns the number of rows `self` has.
    /// 
    /// Always returns 1.
    #[inline]
    pub const fn rows(&self) -> u32 {
        1
    }

    /// Returns the number of columns `self` has.
    #[inline]
    pub const fn cols(&self) -> u32 {
        self.0.cols()
    }

    /// Returns a reference to inner data of `self`.
    #[inline]
    pub const fn inner(&self) -> &[i32] {
        &self.0.inner()
    }

    /// Returns a reference to inner data of `self`.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut [i32] {
        self.0.inner_mut()
    }

    /// Returns the matrix representing `self`.
    #[inline]
    pub const fn matrix(&self) -> &Matrix {
        self.0.matrix()
    }
}

impl ToString for BinaryVector {
    fn to_string(&self) -> String {
        let mut res = String::with_capacity(self.0.inner().len());
        for num in self.inner() {
            res.push(match num {
                0 => '0',
                1 => '1',
                _ => panic!("Non binary digit encountered in a binary vector")
            })
        }
        res
    }
}

impl AsRef<BinaryVector> for BinaryVector {
    #[inline]
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl AsRef<Vector> for BinaryVector {
    #[inline]
    fn as_ref(&self) -> &Vector {
        &self.0
    }
}

impl AsRef<Matrix> for BinaryVector {
    #[inline]
    fn as_ref(&self) -> &Matrix {
        &self.0.as_ref()
    }
}

impl AsRef<[i32]> for BinaryVector {
    #[inline]
    fn as_ref(&self) -> &[i32] {
        &self.inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_vector_from_binary_bytes() {
        assert_eq!(BinaryVector::from_binary_bytes(""), None);

        assert_eq!(BinaryVector::from_binary_bytes(" "), None);

        assert_eq!(BinaryVector::from_binary_bytes("G"), None);

        assert_eq!(BinaryVector::from_binary_bytes("1010G"), None);

        assert_eq!(BinaryVector::from_binary_bytes("2100"), None);

        assert_eq!(BinaryVector::from_binary_bytes("1"), Some(BinaryVector(Vector::from([1].to_vec()))));

        assert_eq!(BinaryVector::from_binary_bytes("0"), Some(BinaryVector(Vector::from([0].to_vec()))));

        assert_eq!(BinaryVector::from_binary_bytes("101010"), Some(BinaryVector(Vector::from([1, 0, 1, 0, 1, 0].to_vec()))));

        assert_eq!(BinaryVector::from_binary_bytes(" 101010"), Some(BinaryVector(Vector::from([1, 0, 1, 0, 1, 0].to_vec()))));
    
        assert_eq!(BinaryVector::from_binary_bytes("101010 "), Some(BinaryVector(Vector::from([1, 0, 1, 0, 1, 0].to_vec()))));

        assert_eq!(BinaryVector::from_binary_bytes(" 1 0     1  0  1  0 "), Some(BinaryVector(Vector::from([1, 0, 1, 0, 1, 0].to_vec()))));
    }

    #[test]
    fn binary_vector_to_string() {
        assert_eq!(BinaryVector::to_string(&BinaryVector(Vector::from([1].to_vec()))), String::from("1"));
    
        assert_eq!(BinaryVector::to_string(&BinaryVector(Vector::from([0].to_vec()))), String::from("0"));
    
        assert_eq!(BinaryVector::to_string(&BinaryVector(Vector::from([0, 1, 0, 0, 1].to_vec()))), String::from("01001"));
    }

    #[test]
    fn binary_vector_dot_product() {
        let vector_1_1 = BinaryVector(Vector::from([2].to_vec()));
        let vector_1_5 = BinaryVector(Vector::from([1, 0, 5, 4, 9].to_vec()));

        assert_eq!(BinaryVector::dot_product(&vector_1_5, &vector_1_1), None);

        assert_eq!(
            BinaryVector::dot_product(&vector_1_1, &vector_1_1), 
            Some(BinaryVector(Vector::from([0].to_vec())))
        );

        assert_eq!(
            BinaryVector::dot_product(&vector_1_1, &vector_1_5),
            Some(BinaryVector(Vector::from([0, 0, 0, 0, 0].to_vec())))
        );
    }

    #[test]
    fn binary_vector_substitute_zeroes() {
        assert_eq!(
            BinaryVector::substitute_zeroes(&BinaryVector(Vector::from([1, 1, 1].to_vec()))),
            Vector::from([1, 1, 1].to_vec())
        );

        assert_eq!(
            BinaryVector::substitute_zeroes(&BinaryVector(Vector::from([0, 0, 0].to_vec()))),
            Vector::from([-1, -1, -1].to_vec())
        );

        assert_eq!(
            BinaryVector::substitute_zeroes(&BinaryVector(Vector::from([0, 1, 0].to_vec()))),
            Vector::from([-1, 1, -1].to_vec())
        );
    }

    #[test]
    fn vector_dot_product() {
        let vector_1_1 = Vector::from([2].to_vec());
        let vector_1_5 = Vector::from([1, 0, 5, 4, 9].to_vec());
        
        assert_eq!(Vector::dot_product(&vector_1_5, &vector_1_1), None);

        assert_eq!(
            Vector::dot_product(&vector_1_1, &vector_1_1), 
            Some(Vector::from([4].to_vec()))
        );

        assert_eq!(
            Vector::dot_product(&vector_1_1, &vector_1_5),
            Some(Vector::from([2, 0, 10, 8, 18].to_vec()))
        );
    }
}