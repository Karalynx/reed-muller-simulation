
use crate::{
    math::{matrix::GenMatrix, vector::BinaryVector}, 
    parameters::Muller
};

use super::reed_muller::{rm_decode, rm_encode, Hadamards};

pub type DecodedLen = u8;

/// Stores split binary vectors according to the [`Muller`] parameter.
/// 
/// - `full_vectors` contains bits that were able to fit into a vector of size `m + 1`.
/// - `rem_vector` contains leftover bits with trailing zeroes and the length of the leftover bits.
/// - `encoded` shows whether the [`SplitVector`] is encoded or not.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SplitVector {
    pub full_vectors: Vec::<BinaryVector>,
    pub rem_vector: Option<(BinaryVector, DecodedLen)>,
    pub encoded: bool
}

impl SplitVector {
    /// Splits a `vec` into vectors of lengths `m + 1`.
    /// 
    /// Returns a split vector.
    pub fn new(vec: &BinaryVector, m: Muller) -> Self {
        let bits = vec.inner();
        let cols = m.rows() as usize;
    
        let rem_bits = bits.len() % cols;
        let len_full_bits = bits.len() - rem_bits;
    
        let mut full_vectors = Vec::<BinaryVector>::with_capacity(len_full_bits >> 3);
        
        let mut current_vec = Vec::<i32>::with_capacity(cols);
        
        for i in 0 .. len_full_bits {
            current_vec.push(bits[i]);

            if current_vec.len() == cols {
                full_vectors.push(unsafe { BinaryVector::from_vec_unchecked(current_vec) });
                current_vec = Vec::<i32>::with_capacity(cols);
            }
        }

        if 0 != rem_bits {
            // The remaining bits will be in a different matrix
            let mut rem_bit_vec = vec![0; cols];
            for i in 0 .. rem_bits {
                rem_bit_vec[i] = bits[len_full_bits + i];
            }

            Self {
                full_vectors, 
                rem_vector: Some((unsafe { BinaryVector::from_vec_unchecked(rem_bit_vec) }, rem_bits as u8)),
                encoded: false
            }
        }
        else {
            Self {
                full_vectors, 
                rem_vector: None,
                encoded: false
            }
        }
    }

    /// Encodes `self` by multiplying each vector by `gen_matrix`.
    pub fn encode(&mut self, gen_matrix: &GenMatrix) {
        if self.encoded {
            return;
        }

        for m in self.full_vectors.iter_mut() {
            *m = rm_encode(&m, &gen_matrix);
        };

        if let Some((m, _)) = self.rem_vector.as_mut() {
            *m = rm_encode(&m, &gen_matrix);
        }

        self.encoded = true;
    }

    /// Decodes `self` using fast Hadamard transform.
    pub fn decode(&mut self, hadamards: &Hadamards) {
        if !self.encoded {
            return;
        }

        for m in self.full_vectors.iter_mut() {
            let (decoded_vec, _) = rm_decode(m, &hadamards);
            *m = decoded_vec;
        };

        if let Some((m, _)) = self.rem_vector.as_mut() {
            let (decoded_vec, _) = rm_decode(m, &hadamards);
            *m = decoded_vec;
        }

        self.encoded = false;
    }

    /// Restores all split parts into one single `BinaryVector`.
    pub fn restore(&self) -> BinaryVector {
        let mut restored = Vec::<i32>::with_capacity(10 * self.full_vectors.len());
        for vec in self.full_vectors.iter() {
            restored.extend_from_slice(vec.inner());
        }

        if let Some((rem, len)) = &self.rem_vector {
            restored.extend_from_slice(match self.encoded {
                true => rem.inner(),
                false => &rem.inner()[0 .. *len as usize],
            })
        }

        unsafe { BinaryVector::from_vec_unchecked(restored) }
    }
    
    /// Restores a message from split parts.
    /// 
    /// Returns an array of restored bytes.
    pub fn to_bytes(&self) -> Box<[u8]> {
        const BYTE_SIZE: usize = u8::BITS as usize;

        let mut byte = 0u8;
        let mut cur_pow = 0usize;
    
        let mut restored_bytes = Vec::<u8>::with_capacity(10 * self.full_vectors.len());
        for matrix in self.full_vectors.iter() {
            for bit in matrix.inner().iter() {
                byte |= (*bit as u8) << (BYTE_SIZE - 1 - cur_pow);
                cur_pow += 1;
    
                if cur_pow == BYTE_SIZE {
                    restored_bytes.push(byte);
                    byte = 0;
                    cur_pow = 0;
                }
            }
        }
    
        if let Some((m, num)) = &self.rem_vector {
            let size = if self.encoded {m.cols() as usize} else {*num as usize};
            for i in 0 .. size {
                byte |= (m.inner()[i] as u8) << (BYTE_SIZE - 1 - cur_pow);
                cur_pow += 1;
    
                if cur_pow == BYTE_SIZE {
                    restored_bytes.push(byte);
                    byte = 0;
                    cur_pow = 0;
                }
            }
        }
    
        restored_bytes.into_boxed_slice()        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_vector_new() {
        assert_eq!(
            SplitVector::new(&BinaryVector::from_bits("1").unwrap(), Muller::new(1).unwrap()),
            SplitVector { 
                full_vectors: unsafe { vec![
                    BinaryVector::from_vec_unchecked(vec![0, 0]),
                    BinaryVector::from_vec_unchecked(vec![1, 1]),
                    BinaryVector::from_vec_unchecked(vec![0, 0]),
                    BinaryVector::from_vec_unchecked(vec![0, 1])
                ]},
                rem_vector: None,
                encoded: false
            },
        );

        assert_eq!(
            SplitVector::new(&BinaryVector::from_bits("1").unwrap(), Muller::new(2).unwrap()),
            SplitVector { 
                full_vectors: unsafe { vec![
                    BinaryVector::from_vec_unchecked(vec![0, 0, 1]),
                    BinaryVector::from_vec_unchecked(vec![1, 0, 0]),
                ]},
                rem_vector: unsafe { Some((BinaryVector::from_vec_unchecked(vec![0, 1, 0]), 2)) },
                encoded: false
            },
        );

        assert_eq!(
            SplitVector::new(&BinaryVector::from_bits("1").unwrap(), Muller::new(7).unwrap()),
            SplitVector { 
                full_vectors: unsafe { vec![
                    BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1])
                ]},
                rem_vector: None,
                encoded: false
            },
        );

        assert_eq!(
            SplitVector::new(&BinaryVector::from_bits("1").unwrap(), Muller::new(10).unwrap()),
            SplitVector { 
                full_vectors: vec![],
                rem_vector: unsafe { Some((BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0]), 8)) },
                encoded: false
            },
        );
    }

    #[test]
    fn split_vector_restore() {
        assert_eq!(
            SplitVector { 
                full_vectors: unsafe { vec![
                    BinaryVector::from_vec_unchecked(vec![0, 0]),
                    BinaryVector::from_vec_unchecked(vec![1, 1]),
                    BinaryVector::from_vec_unchecked(vec![0, 0]),
                    BinaryVector::from_vec_unchecked(vec![0, 1])
                ]},
                rem_vector: None,
                encoded: false
            }.restore(),
            unsafe { BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1]) }
        );

        assert_eq!(
            SplitVector { 
                full_vectors: unsafe { vec![
                    BinaryVector::from_vec_unchecked(vec![0, 0, 1]),
                    BinaryVector::from_vec_unchecked(vec![1, 0, 0]),
                ]},
                rem_vector: unsafe { Some((BinaryVector::from_vec_unchecked(vec![0, 1, 0]), 2))},
                encoded: false
            }.restore(),
            unsafe { BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1]) }
        );

        assert_eq!(
            SplitVector { 
                full_vectors: unsafe { vec![
                    BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1]),
                ]},
                rem_vector: None,
                encoded: false
            }.restore(),
            unsafe { BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1]) }
        );

        assert_eq!(
            SplitVector { 
                full_vectors: vec![],
                rem_vector: unsafe { Some((BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0]), 8))},
                encoded: false
            }.restore(),
            unsafe { BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1]) }
        );

        assert_eq!(
            SplitVector { 
                full_vectors: vec![],
                rem_vector: unsafe { Some((BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]), 8))},
                encoded: false
            }.restore(),
            unsafe { BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1]) }
        );


        assert_eq!(
            SplitVector { 
                full_vectors: unsafe { vec![
                    BinaryVector::from_vec_unchecked(vec![0, 0, 1]),
                    BinaryVector::from_vec_unchecked(vec![1, 0, 0]),
                ]},
                rem_vector: unsafe { Some((BinaryVector::from_vec_unchecked(vec![0, 1, 0]), 2))},
                encoded: true
            }.restore(),
            unsafe { BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1, 0]) }
        );

        assert_eq!(
            SplitVector { 
                full_vectors: unsafe { vec![
                    BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1]),
                ]},
                rem_vector: None,
                encoded: true
            }.restore(),
            unsafe { BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1]) }
        );

        assert_eq!(
            SplitVector { 
                full_vectors: vec![],
                rem_vector: unsafe { Some((BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0]), 8))},
                encoded: true
            }.restore(),
            unsafe { BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0]) }
        );

        assert_eq!(
            SplitVector { 
                full_vectors: vec![],
                rem_vector: unsafe { Some((BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]), 8))},
                encoded: true
            }.restore(),
            unsafe { BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]) }
        );
    }

    #[test]
    fn split_vector_to_bytes() {
        assert_eq!(
            SplitVector { 
                full_vectors: unsafe { vec![
                    BinaryVector::from_vec_unchecked(vec![0, 0]),
                    BinaryVector::from_vec_unchecked(vec![1, 1]),
                    BinaryVector::from_vec_unchecked(vec![0, 0]),
                    BinaryVector::from_vec_unchecked(vec![0, 1])
                ]},
                rem_vector: None,
                encoded: false
            }.to_bytes(),
            vec![49].into()
        );

        assert_eq!(
            SplitVector { 
                full_vectors: unsafe { vec![
                    BinaryVector::from_vec_unchecked(vec![0, 0, 1]),
                    BinaryVector::from_vec_unchecked(vec![1, 0, 0]),
                ]},
                rem_vector: unsafe { Some((BinaryVector::from_vec_unchecked(vec![0, 1, 0]), 2))},
                encoded: false
            }.to_bytes(),
            vec![49].into()
        );

        assert_eq!(
            SplitVector { 
                full_vectors: unsafe { vec![
                    BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1]),
                ]},
                rem_vector: None,
                encoded: false
            }.to_bytes(),
            vec![49].into()
        );

        assert_eq!(
            SplitVector { 
                full_vectors: vec![],
                rem_vector: unsafe { Some((BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0]), 8))},
                encoded: false
            }.to_bytes(),
            vec![49].into()
        );

        assert_eq!(
            SplitVector { 
                full_vectors: vec![],
                rem_vector: unsafe { Some((BinaryVector::from_vec_unchecked(vec![0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]), 8))},
                encoded: false
            }.to_bytes(),
            vec![49].into()
        );

        assert_eq!(
            SplitVector { 
                full_vectors: unsafe { vec![
                    BinaryVector::from_vec_unchecked(vec![0, 0, 1]),
                    BinaryVector::from_vec_unchecked(vec![1, 0, 0]),
                    BinaryVector::from_vec_unchecked(vec![0, 1, 0]),
                    BinaryVector::from_vec_unchecked(vec![0, 1, 1]),
                    BinaryVector::from_vec_unchecked(vec![0, 0, 0]),
                    BinaryVector::from_vec_unchecked(vec![1, 0, 0]),
                    BinaryVector::from_vec_unchecked(vec![1, 1, 0]),
                    BinaryVector::from_vec_unchecked(vec![0, 0, 1]),
                    BinaryVector::from_vec_unchecked(vec![0, 0, 1]),
                    BinaryVector::from_vec_unchecked(vec![1, 0, 0])
                ]},
                rem_vector: unsafe { Some((BinaryVector::from_vec_unchecked(vec![0, 1, 0]), 2))},
                encoded: false
            }.to_bytes(),
            vec![49, 49, 49, 49].into()
        );
    }

}