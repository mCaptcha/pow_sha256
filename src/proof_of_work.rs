use crate::network_byte_order::Ne;
use sha2::{digest::FixedOutput, Digest, Sha256};
use std::marker::PhantomData;

pub struct ProofOfWork<T: Ne> {
    proof: u128,
    _spook: PhantomData<T>,
}

impl<T: Ne> ProofOfWork<T> {
    /// Prove work over T.
    ///
    /// Make sure difficulty is not too high. A 64 bit difficulty, for example, takes a long time
    /// on a general purpose processor.
    pub fn prove_work(t: &T, difficulty: u32) -> ProofOfWork<T> {
        let v = t.serialize_to_vec();
        Self::prove_work_serialized(&v, difficulty)
    }

    /// Prove work on an already serialized item of type T.
    /// The input is assumed to be serialized using network byte order.
    ///
    /// Make sure difficulty is not too high. A 64 bit difficulty, for example, takes a long time
    /// on a general purpose processor.
    pub fn prove_work_serialized(prefix: &[u8], difficulty: u32) -> ProofOfWork<T> {
        debug_assert!(difficulty <= 256);
        let prefix_sha = Sha256::new().chain(prefix);
        let mut n = 0;
        while score(prefix_sha.clone(), n) < difficulty {
            n += 1;
        }
        ProofOfWork {
            proof: n,
            _spook: PhantomData,
        }
    }

    /// Calculate the pow score of t and self.
    pub fn score(&self, t: &T) -> u32 {
        let v = t.serialize_to_vec();
        self.score_serialized(&v)
    }

    /// Calculate the pow score of an already serialized T and self.
    /// The input is assumed to be serialized using network byte order.
    pub fn score_serialized(&self, target: &[u8]) -> u32 {
        score(Sha256::new().chain(target), self.proof)
    }
}

fn score(prefix_sha: Sha256, proof: u128) -> u32 {
    leading_zeros(
        prefix_sha
            .chain(&proof.to_be_bytes()) // to_be_bytes() converts to network endian
            .fixed_result()
            .as_slice(),
    )
}

fn leading_zeros(inp: &[u8]) -> u32 {
    let mut ret = 0;
    for n in inp {
        if n == &0 {
            ret += 8;
        } else {
            ret += n.leading_zeros();
            break;
        }
    }
    return ret;
}

#[cfg(test)]
mod test {
    use super::*;

    const DIFFICULTY: u32 = 10;

    #[test]
    fn base_functionality() {
        // Let's prove we did work targeting a phrase.
        let phrase = b"Corver bandar palladianism retroform.".to_vec();
        let pw = ProofOfWork::prove_work(&phrase, DIFFICULTY);
        assert!(pw.score(&phrase) >= DIFFICULTY);
    }
}
