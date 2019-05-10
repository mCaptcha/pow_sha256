use serde::{Deserialize, Serialize};
use sha2::{digest::FixedOutput, Digest, Sha256};
use std::marker::PhantomData;

const SALT: &'static str = "35af8f4890981391c191e6df45b5f780812ddf0213f29299576ac1c98e18173e";

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub struct Pow<T: Serialize> {
    proof: u128,
    _spook: PhantomData<T>,
}

// prove_work and score could theoretically be without allocations, by serializing to a Write
// implementaion that performs sha256 lazily.
// `impl io::Write for sha2::Sha256 { ... }`

impl<T: Serialize> Pow<T> {
    /// Prove work over T.
    ///
    /// Make sure difficulty is not too high. A 64 bit difficulty, for example, takes a long time
    /// on a general purpose processor.
    ///
    /// Returns bincode::Error if serialization fails.
    pub fn prove_work(t: &T, difficulty: u32) -> bincode::Result<Pow<T>> {
        bincode_cfg()
            .serialize(t)
            .map(|v| Self::prove_work_serialized(&v, difficulty))
    }

    /// Prove work on an already serialized item of type T.
    /// The input is assumed to be serialized using network byte order.
    ///
    /// Make sure difficulty is not too high. A 64 bit difficulty, for example, takes a long time
    /// on a general purpose processor.
    pub fn prove_work_serialized(prefix: &[u8], difficulty: u32) -> Pow<T> {
        debug_assert!(difficulty <= 256);
        let prefix_sha = Sha256::new().chain(SALT).chain(prefix);
        let mut n = 0;
        while score(prefix_sha.clone(), n) < difficulty {
            n += 1;
        }
        Pow {
            proof: n,
            _spook: PhantomData,
        }
    }

    /// Calculate the pow score of t and self.
    pub fn score(&self, t: &T) -> bincode::Result<u32> {
        bincode_cfg()
            .serialize(t)
            .map(|v| self.score_serialized(&v))
    }

    /// Calculate the pow score of an already serialized T and self.
    /// The input is assumed to be serialized using network byte order.
    pub fn score_serialized(&self, target: &[u8]) -> u32 {
        score(Sha256::new().chain(SALT).chain(target), self.proof)
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

fn bincode_cfg() -> bincode::Config {
    let mut cfg = bincode::config();
    cfg.big_endian();
    cfg
}

#[cfg(test)]
mod test {
    use super::*;

    const DIFFICULTY: u32 = 10;

    #[test]
    fn base_functionality() {
        // Let's prove we did work targeting a phrase.
        let phrase = b"Corver bandar palladianism retroform.".to_vec();
        let pw = Pow::prove_work(&phrase, DIFFICULTY).unwrap();
        assert!(pw.score(&phrase).unwrap() >= DIFFICULTY);
    }

    #[test]
    fn double_pow() {
        let phrase = "Corver bandar palladianism retroform.".to_owned();
        let pow = Pow::prove_work(&phrase, DIFFICULTY).unwrap();
        let powpow: Pow<Pow<String>> = Pow::prove_work(&pow, DIFFICULTY).unwrap();
        assert!(pow.score(&phrase).unwrap() >= DIFFICULTY);
        assert!(powpow.score(&pow).unwrap() >= DIFFICULTY);
    }

    #[test]
    fn ser_de() {
        let target: u8 = 1;
        let pw = Pow::prove_work(&target, DIFFICULTY).unwrap();
        let message: (u8, Pow<u8>) = (target, pw);
        let message_ser = bincode_cfg().serialize(&message).unwrap();
        let recieved_message: (u8, Pow<u8>) = bincode_cfg().deserialize(&message_ser).unwrap();
        assert_eq!(recieved_message, message);
        assert!(message.1.score(&message.0).unwrap() >= DIFFICULTY);
    }
}
