use serde::{Deserialize, Serialize};
use sha2::{digest::FixedOutput, Digest, Sha256};
use std::marker::PhantomData;

const SALT: &str = "79ziepia7vhjgviiwjhnend3ofjqocsi2winc4ptqhmkvcajihywxcizewvckg9h6gs4j83v9";

/// Proof of Work over concrete type T. T can be any type that implements serde::Serialize.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct PoW<T> {
    pub nonce: u64,
    pub result: String,
    _spook: PhantomData<T>,
}

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
            result: result.to_string(),
            _spook: PhantomData,
        }
    }

    /// Calculate the PoW score with the provided input T.
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

    /// Verifies that the PoW is indeed generated out of the phrase provided.
    pub fn is_valid_proof(&self, t: &T) -> bool {
        match self.calculate(t) {
            Ok(res) => return if self.result == res.to_string() {true} else {false},
            Err(_) => return false 
        }
    }

    /// Checks if the PoW result is of sufficient difficulty
    pub fn is_sufficient_difficulty(&self, target_diff: u128) -> bool {
        match self.result.parse::<u128>() {
            Ok(res) => return res >= target_diff,
            Err(_) =>  return false
        }
    }
}

fn score(prefix_sha: Sha256, nonce: u64) -> u128 {
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
        let phrase = b"Ex nihilo nihil fit.".to_vec();
        let pw = PoW::prove_work(&phrase, DIFFICULTY).unwrap();
        assert!(pw.calculate(&phrase).unwrap() >= DIFFICULTY);
        assert!(pw.is_valid_proof(&phrase));
        assert!(pw.is_sufficient_difficulty(DIFFICULTY));
    }

    #[test]
    fn double_pow() {
        let phrase = "Ex nihilo nihil fit.".to_owned();
        let pw = PoW::prove_work(&phrase, DIFFICULTY).unwrap();
        let pwpw: PoW<PoW<String>> = PoW::prove_work(&pw, DIFFICULTY).unwrap();
        assert!(pw.calculate(&phrase).unwrap() >= DIFFICULTY);
        assert!(pwpw.calculate(&pw).unwrap() >= DIFFICULTY);
        assert!(pw.is_sufficient_difficulty(DIFFICULTY));
        assert!(pwpw.is_sufficient_difficulty(DIFFICULTY));
        assert!(pw.is_valid_proof(&phrase));
        assert!(pwpw.is_valid_proof(&pw));
    }

    #[test]
    fn is_not_valid_proof() {
        let phrase = "Ex nihilo nihil fit.".to_owned();
        let phrase2 = "Omne quod movetur ab alio movetur.".to_owned();
        let pw = PoW::prove_work(&phrase, DIFFICULTY).unwrap();
        let pw2 = PoW::prove_work(&phrase2, DIFFICULTY).unwrap();
        assert!(!pw.is_valid_proof(&phrase2));
        assert!(!pw2.is_valid_proof(&phrase));
    }

    #[test]
    fn serialization_test() {
        let target: u8 = 1;
        let pw = PoW::prove_work(&target, DIFFICULTY).unwrap();
        let message: (u8, PoW<u8>) = (target, pw);
        let message_ser = bincode_cfg().serialize(&message).unwrap();
        let recieved_message: (u8, PoW<u8>) = bincode_cfg().deserialize(&message_ser).unwrap();
        assert_eq!(recieved_message, message);
        assert!(message.1.is_sufficient_difficulty(DIFFICULTY));
        assert!(message.1.is_valid_proof(&target));
    }
}
