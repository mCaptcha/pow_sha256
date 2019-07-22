# PoW-SHA256

Rust crate which generates SHA256 Proofs of Work on serializable datatypes. 

Any type that implements `serde::Deserialize` can be used.

This is a fork of the [`Pow` library](https://github.com/bddap/pow) by bddap with some new additions. Primary of these being:

- PoW datatype now saves the calculation result to be used for checking proof validity given input
- `is_valid_proof` method to do the above mentioned

Other small changes have also been included of various importance but mostly just stylistic/ease of use improvements.

# Examples

Prove we did work targeting a phrase.

```rust
use PoW::PoW;

// very easy mode
let difficulty = u128::max_value() - u128::max_value() / 2;

let phrase = b"Phrase to tag.".to_vec();
let pw = PoW::prove_work(&phrase, difficulty).unwrap();
assert!(pw.score(&phrase).unwrap() >= difficulty);
```

Prove more difficult work. This time targeting a time.

```rust
// more diffcult, takes around 100_000 hashes to generate proof
let difficulty = u128::max_value() - u128::max_value() / 100_000;

let now: u64 = get_unix_time_seconds();
let pw = PoW::prove_work(&now, difficulty).unwrap();
assert!(pw.score(&now).unwrap() >= difficulty);
```

Define a blockchain block.

```rust
struct Block<T> {
    prev: [u8; 32], // hash of last block
    payload: T,     // generic data
    proof_of_work: PoW<([u8; 32], T)>,
}
```

# Hashing Scheme

SHA256 is calculated over the concatenation of the:
- SALT
- Serialized Input `T` 
- Nonce

The first 16 bytes of the resulting hash are interpreted as a 128 bit unsigned integer.

A randomly generated constant, `SALT`, is used as prefix to prevent PoW reuse from other systems such as proof of work blockchains.

# Choosing a difficulty setting.

Difficulty settings are usually best adjusted dynamically a la bitcoin.

To manually select a difficulty, choose the average number of hashes required.

```rust
fn difficulty(average: u128) -> u128 {
    debug_assert_ne!(average, 0, "It is impossible to prove work in zero attempts.");
    let m = u128::max_value();
    m - m / average
}
```

Conversely, to calculate probable number of hashes required to satisfy a given minimum
difficulty.

```rust
fn average(difficulty: u128) -> u128 {
    let m = u128::max_value();
    if difficulty == m {
        return m;
    } 
    m / (m - difficulty)
}
```

# License

This project is dual-licensed under `Apache License Version 2.0` & `MIT license`.
