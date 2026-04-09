
use crate::math::matrix::*;
use crate::math::vector::*;
use crate::parameters::Muller;

/// Stores precalculated hadamard matrices.
#[derive(Debug, Clone)]
pub struct Hadamards {
    matrices: Box<[Matrix]>,
    m: Muller
}

impl Hadamards {
    /// Precalculate hadamard matrices with parameter `m`.
    /// 
    /// Returns an array of hadamard matrices of size \[1; m].
    pub fn new(m: Muller) -> Self {
        let muller = m.get() as u32;

        let mut matrices = Vec::<Matrix>::with_capacity(muller as usize);
        let parity = Matrix::parity();

        for i in 1 .. (muller + 1) {
            let identity = Matrix::identity(2u16.pow(muller - i)).unwrap();
            let identity2 = Matrix::identity(2u16.pow(i - 1)).unwrap();

            matrices.push(identity
                .kronecher_product(&parity)
                .kronecher_product(&identity2)
            );
        }

        Self {  
            matrices: matrices.into_boxed_slice(),
            m
        }
    }

    /// Returns a reference to inner data of `self`.
    #[inline]
    pub const fn matrices(&self) -> &Box<[Matrix]> {
        &self.matrices
    }

    /// Returns the muller parameter of `self`.
    #[inline]
    pub const fn muller(&self) -> Muller {
        self.m
    }

}

/// Encodes `vector` using `RM(1, m)` generated `gen_matrix`.
/// 
/// Returns an encoded [`BinaryVector`].
/// 
/// # Safety
///
/// `gen_matrix` must have the same amount of rows as `vector` has columns.
#[inline]
pub fn rm_encode(vector: &BinaryVector, gen_matrix: &GenMatrix) -> BinaryVector {
    unsafe { vector.dot_product_unchecked(gen_matrix) }
}

/// Decodes a message from weights.
/// - idx is in index of the largest absolute weight
/// - num is the largest absolute weight
/// - m is the Muller parameter
/// 
/// Returns a decoded binary vector
fn weights_to_vector(idx: i32, num: i32, m: Muller) -> BinaryVector {
    let mut bits: Vec<i32> = vec![0; m.rows() as usize];
    bits[0] = if num < 0 {0} else {1};
    
    for i in 0 .. m.get() as usize {
        bits[i + 1] = (idx >> i) & 1;
    }
    
    unsafe { BinaryVector::from_vec_unchecked(bits) }
}

/// Decodes `vector` encoded with `RM(1, m)`.
/// 
/// Returns a decoded `vector` and the weights calculated.
/// 
/// # Safety
///
/// `hadamards` must have the same `m` parameter as the encoded `vector`.
pub fn rm_decode(vector: &BinaryVector, hadamards: &Hadamards) -> (BinaryVector, Vector) {
    let mut weights = vector.substitute_zeroes();
    for h in hadamards.matrices() {
        weights = unsafe { weights.dot_product_unchecked(&h) };
    }

    let (idx, num) = weights.inner()
        .iter()
        .enumerate()
        .max_by_key(|(_, x)| x.abs()
    ).unwrap();

    // idx will always be < 11.
    (weights_to_vector(idx as i32, *num, hadamards.muller()), weights)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hadamards_new() {
        assert_eq!(
            *Hadamards::new(Muller::new(1).unwrap()).matrices(),
            vec![Matrix::new(2, 2, [1, 1,  1, -1].into()).unwrap()].into_boxed_slice()
        );

        assert_eq!(
            *Hadamards::new(Muller::new(2).unwrap()).matrices(),
            vec![
                Matrix::new(4, 4, [1, 1, 0, 0,  1, -1, 0, 0,  0, 0, 1, 1,  0, 0, 1, -1].into()).unwrap(),
                Matrix::new(4, 4, [1, 0, 1, 0,  0, 1, 0, 1,  1, 0, -1, 0,  0, 1, 0, -1].into()).unwrap()
            ].into_boxed_slice()
        );
    }
    
    #[test]
    fn reed_muller_encode() {
        let valid1 = rm_encode(&BinaryVector::from_binary_bytes("1100").unwrap(), &GenMatrix::new(Muller::new(3).unwrap()));
        assert_eq!(valid1, BinaryVector::from_binary_bytes("10101010").unwrap()); 
    }

    #[test]
    fn reed_muller_decode() {
        let mut vector = BinaryVector::from_binary_bytes("10101011").unwrap();
        let decoded = rm_decode(
            &mut vector,
            &Hadamards::new(Muller::new(3).unwrap())
        );
        
        assert_eq!(decoded.0, BinaryVector::from_binary_bytes("1100").unwrap());
    }
}
