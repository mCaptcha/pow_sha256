use serde::{Deserialize, Serialize};
use sha2::{digest::FixedOutput, Digest, Sha256};
use std::marker::PhantomData;

const SALT: &str = "79ziepia7vhjgviiwjhnend3ofjqocsi2winc4ptqhmkvcajihywxcizewvckg9h6gs4j83v9";

/// Proof of Work over concrete type T. T can be any type that implements serde::Serialize.
#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub struct PoW<T> {
    nonce: u128,
    result: u128,
    _spook: PhantomData<T>,
}

// prove_work and calculate could theoretically be without allocations, by serializing to a Write
// implementaion that performs sha256 lazily.
// `impl io::Write for sha2::Sha256 { ... }`

impl<T: Serialize> PoW<T> {
    /// Create Proof of Work over item of type T.
    ///
    /// Make sure difficulty is not too high. A 64 bit difficulty, for example, takes a long time
    /// on a general purpose processor.
    ///
    /// Returns bincode::Error if serialization fails.
    pub fn prove_work(t: &T, difficulty: u128) -> bincode::Result<PoW<T>> {
        bincode_cfg()
            .serialize(t)
            .map(|v| Self::prove_work_serialized(&v, difficulty))
    }

    /// Create Proof of Work on an already serialized item of type T.
    /// The input is assumed to be serialized using network byte order.
    ///
    /// Make sure difficulty is not too high. A 64 bit difficulty, for example, takes a long time
    /// on a general purpose processor.
    pub fn prove_work_serialized(prefix: &[u8], difficulty: u128) -> PoW<T> {
        let prefix_sha = Sha256::new().chain(SALT).chain(prefix);
        let mut n = 0;
        let mut result = 0;
        while result < difficulty {
            n += 1;
            result = score(prefix_sha.clone(), n);
        }
        PoW {
            nonce: n,
            result: result,
            _spook: PhantomData,
        }
    }

    /// Calculate the PoW score of t and self.
    pub fn calculate(&self, t: &T) -> bincode::Result<u128> {
        bincode_cfg()
            .serialize(t)
            .map(|v| self.calculate_serialized(&v))
    }

    /// Calculate the PoW score of an already serialized T and self.
    /// The input is assumed to be serialized using network byte order.
    pub fn calculate_serialized(&self, target: &[u8]) -> u128 {
        score(Sha256::new().chain(SALT).chain(target), self.nonce)
    }
}

fn score(prefix_sha: Sha256, nonce: u128) -> u128 {
    first_bytes_as_u128(
        prefix_sha
            .chain(&nonce.to_be_bytes()) // to_be_bytes() converts to network endian
            .fixed_result()
            .as_slice(),
    )
}

/// # Panics
///
/// panics if inp.len() < 16
fn first_bytes_as_u128(inp: &[u8]) -> u128 {
    bincode_cfg().deserialize(&inp).unwrap()
}

fn bincode_cfg() -> bincode::Config {
    let mut cfg = bincode::config();
    cfg.big_endian();
    cfg
}

#[cfg(test)]
mod test {
    use super::*;

    const DIFFICULTY: u128 = 0xff000000000000000000000000000000;

    #[test]
    fn base_functionality() {
        // Let's prove we did work targeting a phrase.
        let phrase = b"Corver bandar palladianism retroform.".to_vec();
        let pw = PoW::prove_work(&phrase, DIFFICULTY).unwrap();
        assert!(pw.calculate(&phrase).unwrap() >= DIFFICULTY);
    }

    #[test]
    fn double_pow() {
        let phrase = "Corver bandar palladianism retroform.".to_owned();
        let PoW = PoW::prove_work(&phrase, DIFFICULTY).unwrap();
        let PoWPoW: PoW<PoW<String>> = PoW::prove_work(&PoW, DIFFICULTY).unwrap();
        assert!(PoW.calculate(&phrase).unwrap() >= DIFFICULTY);
        assert!(PoWPoW.calculate(&PoW).unwrap() >= DIFFICULTY);
    }

    #[test]
    fn ser_de() {
        let target: u8 = 1;
        let pw = PoW::prove_work(&target, DIFFICULTY).unwrap();
        let message: (u8, PoW<u8>) = (target, pw);
        let message_ser = bincode_cfg().serialize(&message).unwrap();
        let recieved_message: (u8, PoW<u8>) = bincode_cfg().deserialize(&message_ser).unwrap();
        assert_eq!(recieved_message, message);
        assert!(message.1.calculate(&message.0).unwrap() >= DIFFICULTY);
    }
}
