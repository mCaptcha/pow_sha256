# PoW_SHA256

Rust crate which generates SHA256 Proofs of Work on serializable datatypes. 

Whether for blockchain-related projects or Hashcash-like schemes, this crate can be used to prove work was done on a given serializable input. The input merely needs to implement `serde::Deserialize` to be used.

This is a fork of the [`pow` library](https://github.com/bddap/pow) by bddap with some new additions. Primary of these being:

- PoW datatype now saves the calculation result to be used for checking proof validity given input
- `is_valid_proof` method to do the above mentioned

Other small changes have also been included of various importance but mostly just stylistic/ease of use improvements.

# Examples

Prove work was done, specifically targeting a phrase.

```rust
use pow_sha256::PoW;

// Very easy difficulty
let difficulty = u128::max_value() - u128::max_value() / 2;

let phrase = b"Phrase to be used.".to_vec();
let pw = PoW::prove_work(&phrase, difficulty).unwrap();

// Asserting that the result is of sufficient difficulty
assert!(pw.is_sufficient_difficulty(difficulty));

// Asserting that the PoW was generated from the provided phrase
assert!(pw.is_valid_proof(&phrase))
```

Prove more difficult work. This time targeting a time.

```rust
// Greater diffculty this time around. Takes around 100,000 hashes to find a nonce of the correct difficulty.
let difficulty = u128::max_value() - u128::max_value() / 100_000;

let now: u64 = get_unix_time_seconds();
let pw = PoW::prove_work(&now, difficulty).unwrap();

// Alternative way to check that the result is of sufficient difficulty
assert!(pw.result >= difficulty);
assert!(pw.is_valid_proof(&phrase))
```


# Hashing Scheme

A randomly generated constant, `SALT`, is used as prefix to prevent PoW reuse from other systems such as proof of work blockchains.

SHA256 is calculated over the concatenation of the:
- SALT
- Serialized Input `T` 
- Nonce

The first 16 bytes of the resulting hash are interpreted as a 128 bit unsigned integer and saved as the final result.


# Choosing a difficulty setting.

Depending on your use case, difficulty settings often are best set dynamically a la bitcoin.

However if your use case requires manual setting then it is trivial to set one yourself. One way to do so is to choose the average number of hashes desired with a function like this:

```rust
fn get_difficulty(average: u128) -> u128 {
    debug_assert_ne!(average, 0, "It is impossible to prove work in zero attempts.");
    let m = u128::max_value();
    m - m / average
}
```

Conversely we can use the same equation to calculate the probable number of hashes required to satisfy a given difficulty:

```rust
fn est_average(difficulty: u128) -> u128 {
    let m = u128::max_value();
    if difficulty == m {
        return m;
    } 
    m / (m - difficulty)
}
```

# License

This project is dual-licensed under `Apache License Version 2.0` & `MIT license`.
