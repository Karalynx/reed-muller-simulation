
use std::fmt::Display;

use crate::parameters::Muller;
use super::vector::Vector;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix {
    rows: u32,
    cols: u32,
    data: Box<[i32]>,
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0 .. self.rows{
            for j in 0 .. self.cols {
                write!(f, "{} ", self.data[((i * self.cols) + j) as usize])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Matrix {
    /// Creates a new matrix with specified `rows` and `cols`, containing `data`.
    /// 
    /// Returns `None` if `data` is empty or specified dimensions don't match its length.
    pub fn new(rows: u16, cols: u16, data: Vec<i32>) -> Option<Self> {
        if data.is_empty() || data.len() != (rows * cols) as usize {
            return None;
        }
        else {
            Some(Self {
                rows: rows as u32,
                cols: cols as u32,
                data: data.into_boxed_slice(),
            })
        }
    }

    /// Creates a new matrix with specified `rows` and `cols`, containing `data`.
    /// 
    /// Returns a new matrix.
    /// 
    /// # Safety
    ///
    /// The caller must ensure that `data` is of length `rows * cols`
    pub unsafe fn new_unchecked(rows: u16, cols: u16, data: Vec<i32>) -> Self {
        debug_assert!(!data.is_empty() && data.len() == rows as usize * cols as usize);
        Self {
            rows: rows as u32,
            cols: cols as u32,
            data: data.into_boxed_slice(),
        }
    }

    /// Creates an identity matrix of size `n`.
    /// 
    /// Returns `None` if `n` is 0.
    pub fn identity(n: u16) -> Option<Self> {
        if n == 0 {
            return None;
        }

        let n = n as usize;

        let mut data = vec![0; n * n].into_boxed_slice();
        let mut idx = 0;
        while idx < data.len() {
            data[idx] = 1;
            idx += n + 1;
        }

        Some(Self {
            rows: n as u32,
            cols: n as u32,
            data,
        })
    }

    /// Creates a 2x2 parity matrix.
    pub fn parity() -> Self {
        let data = Box::new([1, 1, 1, -1]);
        Self { rows: 2, cols: 2, data }
    }

    /// Returns the number of rows `self` has.
    #[inline]
    pub const fn rows(&self) -> u32 {
        self.rows
    }

    /// Returns the number of columns `self` has.
    #[inline]
    pub const fn cols(&self) -> u32 {
        self.cols
    }

    /// Returns a reference to inner data of `self`.
    #[inline]
    pub const fn inner(&self) -> &[i32] {
        &self.data
    }

    /// Returns a mutable reference inner data of `self`.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut [i32] {
        &mut self.data
    }

    /// Multiplies each element of the MxN `self` matrix by the PxQ `other` matrix.
    ///
    /// Returns a PMxQN matrix.
    pub fn kronecher_product(&self, other: &Matrix) -> Self {
        let (rows, cols) = (self.rows * other.rows, self.cols * other.cols);

        let (self_rows, self_cols) = (self.rows as usize, self.cols as usize);
        let (other_rows, other_cols) = (other.rows as usize, other.cols as usize);

        let mut data = vec![0; (rows * cols) as usize].into_boxed_slice();
        for self_row in 0 .. self_rows {
            for self_col in 0 .. self_cols {
                
                let self_val = self.data[(self_row * self_cols) + self_col];
                // Calculate the beginning index of the submatrix
                let submatrix_start = (self_row * other_rows * cols as usize) + (self_col * other_cols);
                
                for other_row in 0 .. other_rows {
                    for other_col in 0 .. other_cols {
                        
                        let other_val = other.data[(other_row * other_cols) + other_col];
                        // Calculate the offset inside the submatrix
                        let offset = (other_row * cols as usize) + other_col;
                        
                        data[submatrix_start + offset] = self_val * other_val;
                    }
                }
            }
        }

        Self { rows, cols, data }
    }
}

impl From<Vec<i32>> for Matrix {
    #[inline]
    fn from(val: Vec<i32>) -> Self {
        Self { 
            rows: 1, 
            cols: val.len() as u32, 
            data: val.into_boxed_slice() 
        }
    }
}

impl From<Box<[i32]>> for Matrix {
    #[inline]
    fn from(val: Box<[i32]>) -> Self {
        Self { 
            rows: 1, 
            cols: val.len() as u32, 
            data: val 
        }
    }
}

impl From<Vector> for Matrix {
    #[inline]
    fn from(val: Vector) -> Self {
        val.0
    }
}

impl AsRef<Matrix> for Matrix {
    #[inline]
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl AsRef<[i32]> for Matrix {
    #[inline]
    fn as_ref(&self) -> &[i32] {
        &self.data
    }
}

/// Generation matrix for RM(1, m)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenMatrix {
    matrix: Matrix,
    m: Muller
}

impl GenMatrix {
    /// Creates a generation matrix for RM(1, m)
    pub fn new(m: Muller) -> Self {
        let (rows, cols) = (m.rows(), m.cols());
        let mut data = vec![1; (rows * cols) as usize].into_boxed_slice();

        let mut interval = 1;
        // Skip the first row as it's already filled with ones
        for row in 1 .. rows {
            
            let mut col = 0;
            while col < cols {
                for _ in 0..interval {
                    data[((row * cols) + col) as usize] = 0;
                    col += 1;
                }

                // Skip the ones
                col += interval;
            }
            
            interval <<= 1;
        }

        Self { matrix: Matrix { rows, cols, data }, m }
    }

    /// Returns the number of rows `self` has.
    #[inline]
    pub const fn rows(&self) -> u32 {
        self.matrix.rows()
    }

    /// Returns the number of columns `self` has.
    #[inline]
    pub const fn cols(&self) -> u32 {
        self.matrix.cols()
    }

    /// Returns a reference to inner data of `self`.
    #[inline]
    pub const fn inner(&self) -> &[i32] {
        &self.matrix.inner()
    }

    /// Returns the matrix representing `self`.
    #[inline]
    pub const fn matrix(&self) -> &Matrix {
        &self.matrix
    }

    /// Returns the muller parameter of `self`.
    #[inline]
    pub const fn muller(&self) -> Muller {
        self.m
    }
}

impl AsRef<GenMatrix> for GenMatrix {
    #[inline]
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl AsRef<Matrix> for GenMatrix {
    #[inline]
    fn as_ref(&self) -> &Matrix {
        &self.matrix
    }
}

impl AsRef<[i32]> for GenMatrix {
    #[inline]
    fn as_ref(&self) -> &[i32] {
        &self.inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_matrix_new() {
        let muller_1 = Muller::new(1).unwrap();
        let muller_2 = Muller::new(2).unwrap();
        let muller_3 = Muller::new(3).unwrap();

        assert_eq!(
            GenMatrix::new(muller_1), 
            GenMatrix { matrix: Matrix { rows: 2, cols: 2, data: [1, 1,  0, 1].into() }, m: muller_1 }
        );

        assert_eq!(
            GenMatrix::new(muller_2), 
            GenMatrix { matrix: Matrix { rows: 3, cols: 4, data: [1, 1, 1, 1,  0, 1, 0, 1,  0, 0, 1, 1].into() }, m: muller_2 }
        );

        assert_eq!(
            GenMatrix::new(muller_3), 
            GenMatrix { matrix: Matrix { rows: 4, cols: 8, data: [1, 1, 1, 1, 1, 1, 1, 1,  0, 1, 0, 1, 0, 1, 0, 1,  0, 0, 1, 1, 0, 0, 1, 1,  0, 0, 0, 0, 1, 1, 1, 1].into() }, m: muller_3 }
        );
    }

    #[test]
    fn matrix_new() {
        assert_eq!(Matrix::new(0, 0, vec![]), None);

        assert_eq!(Matrix::new(0, 1, vec![]), None);

        assert_eq!(Matrix::new(1, 0, vec![]), None);

        assert_eq!(Matrix::new(1, 1, vec![]), None);

        assert_eq!(Matrix::new(1, 2, vec![1]), None);

        assert_eq!(Matrix::new(2, 1, vec![1]), None);

        assert_eq!(
            Matrix::new(1, 1, vec![1]), 
            Some(Matrix { rows: 1, cols: 1, data: [1].into() })
        );

        assert_eq!(
            Matrix::new(2, 1, vec![1, 2]), 
            Some(Matrix { rows: 2, cols: 1, data: [1, 2].into() })
        );

        assert_eq!(
            Matrix::new(2, 2, vec![1, 2, 3, 4]), 
            Some(Matrix { rows: 2, cols: 2, data: [1, 2, 3, 4].into() })
        );
    }

    #[test]
    fn matrix_identity() {
        assert_eq!(Matrix::identity(0), None);

        assert_eq!(
            Matrix::identity(1), 
            Some(Matrix { rows: 1, cols: 1, data: [1].into() })
        );

        assert_eq!(
            Matrix::identity(2), 
            Some(Matrix { rows: 2, cols: 2, data: [1, 0, 0, 1].into() })
        );

        assert_eq!(
            Matrix::identity(3), 
            Some(Matrix { rows: 3, cols: 3, data: [1, 0, 0,  0, 1, 0,  0, 0, 1].into() })
        );
    }

    #[test]
    fn matrix_parity() {
        assert_eq!(Matrix::parity(), Matrix { rows: 2, cols: 2, data: [1, 1, 1, -1].into() })
    }

    #[test]
    fn matrix_kronecher_product() {
        let matrix_1_1 = Matrix { rows: 1, cols: 1, data: [2].into() };
        let matrix_1_5 = Matrix { rows: 1, cols: 5, data: [1, 0, 5, 4, 9].into() };

        let matrix_2_2 = Matrix { rows: 2, cols: 2, data: [2, 5,  7, 10].into() };
        let matrix_2_3 = Matrix { rows: 2, cols: 3, data: [6, 3, 9,  2, 1, 5].into() };
        
        assert_eq!(
            Matrix::kronecher_product(&matrix_1_1, &matrix_1_1), 
            Matrix { rows: 1, cols: 1, data: [4].into() }
        );

        assert_eq!(
            Matrix::kronecher_product(&matrix_2_3, &matrix_1_1), 
            Matrix { rows: 2, cols: 3, data: [12, 6, 18,  4, 2, 10].into() }
        );

        assert_eq!(
            Matrix::kronecher_product(&matrix_1_5, &matrix_2_2), 
            Matrix { rows: 2, cols: 10, data: [2, 5, 0, 0, 10, 25, 8, 20, 18, 45,  7, 10, 0, 0, 35, 50, 28, 40, 63, 90].into() }
        );
    }
}
