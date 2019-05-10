//! Sha256 based proof of work over a typed piece of data.
//!
//! Any type that implementes serde::Deserialize can be tagged with a proof of work.
//!
//! # Examples
//!
//! Prove we did work targeting a phrase.
//!
//! ```
//! use pow::Pow;
//! let difficulty = u128::max_value() / 2; // very easy mode
//! let phrase = b"Phrase to tag.".to_vec();
//! let pw = Pow::prove_work(&phrase, difficulty).unwrap();
//! assert!(pw.score(&phrase).unwrap() >= difficulty);
//! ```
//!
//! Prove more difficult work. This time targeting a time.
//!
//! ```
//! # fn get_unix_time_seconds() -> u64 {
//! #     use std::time::{Duration, SystemTime};
//! #     SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
//! # }
//! # use pow::Pow;
//! let difficulty = u128::max_value() / 10_000 * 9_999; // more diffcult, around 10_000 hashes
//! let now: u64 = get_unix_time_seconds();
//! let pw = Pow::prove_work(&now, difficulty).unwrap();
//! assert!(pw.score(&now).unwrap() >= difficulty);
//! ```

//! Expected computional cost scales something like
//! O(u128::max_value() / (u128::max_value() - difficulty))

mod proof_of_work;

pub use proof_of_work::Pow;
