#![no_main]
use libfuzzer_sys::fuzz_target;
use pow_sha256::dev::score;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, arbitrary::Arbitrary)]
struct Data {
    salt: Vec<u8>,
    message: Vec<u8>,
    count: u64,
}

fuzz_target!(|data: Data| {
    // fuzzed code goes here
    let prefix_sha = Sha256::new().chain(&data.salt).chain(&data.message);
    let _result = score(prefix_sha.clone(), data.count);
});
