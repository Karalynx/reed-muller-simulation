
use rand::{random, Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;

use crate::{math::vector::BinaryVector, parameters::Probability};
use super::{channel_errors::ChannelErrors, split_vector::SplitVector};

pub trait Send {
    /// Sends multiple `vectors` through the channel.
    fn send_multiple(&mut self, vectors: &mut SplitVector);

    /// Sends a single `vector` through the channel.
    /// 
    /// Returns an array of error indexes.
    fn send_single(&mut self, vector: &mut BinaryVector) -> ChannelErrors;
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub p: Probability,
    pub rng: ChaCha12Rng
}

impl Channel {
    pub fn new(p: Probability) -> Self {
        Self {
            p,
            rng: ChaCha12Rng::from_seed(random())
        }
    }
}

/// Corrupts a binary vector by turning each 0 to 1 and vice versa with a chance of `p`.
/// 
/// Returns an array of error indexes.
pub fn corrupt_with_errors(vec: &mut BinaryVector, p: Probability, rng: &mut ChaCha12Rng) -> ChannelErrors {
    let p = p.get();

    let mut errors = Vec::<usize>::new();
    for (idx, bit) in vec.inner_mut().iter_mut().enumerate() {
        if rng.gen_range(0.0f32 .. 1.0f32) < p {
            errors.push(idx);
            *bit ^= 1;
        }
    }

    ChannelErrors::from(errors)
}

/// Corrupts a binary vector by turning each 0 to 1 and vice versa with a chance of `p`.
pub fn corrupt(vec: &mut BinaryVector, p: Probability, rng: &mut ChaCha12Rng) {
    let p = p.get();

    for bit in vec.inner_mut() {
        if rng.gen_range(0.0f32 .. 1.0f32) < p {
            *bit ^= 1;
        }
    }
}

impl Send for Channel {
    fn send_multiple(&mut self, vectors: &mut SplitVector) {
        for m in vectors.full_vectors.iter_mut() {
            corrupt(m, self.p, &mut self.rng);
        }
        if let Some((m, _)) = vectors.rem_vector.as_mut() {
            corrupt(m, self.p, &mut self.rng);
        }
    }

    #[inline]
    fn send_single(&mut self, vector: &mut BinaryVector) -> ChannelErrors {
        corrupt_with_errors(vector, self.p, &mut self.rng)
    }
}