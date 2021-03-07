<div align="center">
  <h1>PoW-SHA256</h1>
  <p>
    <strong>PoW-SHA256 - SHA256 based Proof-of-Work</strong>
  </p>

[![Documentation](https://img.shields.io/badge/docs-master-blue)](https://mcaptcha.github.io/pow_sha256/pow_sha256/index.html)
![CI (Linux)](<https://github.com/mCaptcha/pow_sha256/workflows/CI%20(Linux)/badge.svg>)
[![dependency status](https://deps.rs/repo/github/mCaptcha/pow_sha256/status.svg)](https://deps.rs/repo/github/mCaptcha/pow_sha256)
<br />
[![codecov](https://codecov.io/gh/mCaptcha/pow_sha256/branch/master/graph/badge.svg)](https://codecov.io/gh/mCaptcha/pow_sha256)

</div>

> pow_sha256's copy of `pow_sha256` by
> [robkorn](https://github.com/robkorn/pow_sha256)
> which is a modified version of [`pow` library](https://github.com/bddap/pow).
> All copyrights belong to the original authors.

Rust crate which generates SHA256 Proofs of Work on serializable datatypes.

Whether for blockchain-related projects or Hashcash-like schemes, this
crate can be used to prove work was done on a given serializable input.
The input merely needs to implement `serde::Deserialize` to be used.

This is a fork of the [`pow` library](https://github.com/bddap/pow) by
[@robkorn](https://github.com/robkorn/pow_sha256)) with some new
additions. Primary of these being:

- PoW datatype now offers a constructor 
- Salt is no longer hard coded into the library, users can provide
  unique salts.

Other small changes have also been included of various importance but
mostly just stylistic/ease of use improvements.

## Examples

Prove work specifically targeting a phrase.

```rust

use pow_sha256::{ConfigBuilder, PoW};

fn main() {
    let config = ConfigBuilder::default()
        .salt("myrandomsaltisnotlongenoug".into())
        .build()
        .unwrap();

    let phrase = "ironmansucks";

    const DIFFICULTY: u128 = u128::MAX / 32;

    let work = config.prove_work(&phrase, DIFFICULTY).unwrap();
    assert!(config.calculate(&work, &phrase).unwrap() >= DIFFICULTY);
    assert!(config.is_valid_proof(&work, &phrase));
    assert!(config.is_sufficient_difficulty(&work, DIFFICULTY));
}
```

Prove more difficult work. This time targeting a time.

```rust
// Greater difficulty this time around. Takes around 100,000 hashes
// to find a nonce of the correct difficulty.


use pow_sha256::{ConfigBuilder, PoW};

fn main() {
    let config = ConfigBuilder::default()
        .salt("myrandomsaltisnotlongenoug".into())
        .build()
        .unwrap();

    let phrase = "ironmansucks";

    const DIFFICULTY: u128 = u128::max_value() - u128::max_value() / 100_000;

    let work = config.prove_work(&phrase, DIFFICULTY).unwrap();

    assert!(config.calculate(&work, &phrase).unwrap() >= DIFFICULTY);
    assert!(config.is_valid_proof(&work, &phrase));
    assert!(config.is_sufficient_difficulty(&work, DIFFICULTY));
}

```

## Hashing Scheme

`SALT` is used as prefix to prevent PoW reuse from other systems such as
proof of work blockchains.

SHA256 is calculated over the concatenation of the:

- SALT
- Serialized Input `T`
- Nonce

The first 16 bytes of the resulting hash are interpreted as a 128 bit
unsigned integer and saved as the final result.

## Choosing a difficulty setting.

Depending on your use case, difficulty settings often are best set
dynamically a la bitcoin.

However if your use case requires manual setting then it is trivial to
set one yourself. One way to do so is to choose the average number of
hashes desired with a function like this:

```rust
fn get_difficulty(average: u128) -> u128 {
    debug_assert_ne!(average, 0, "It is impossible to prove work in zero attempts.");
    let m = u128::max_value();
    m - m / average
}
```

Conversely we can use the same equation to calculate the probable number
of hashes required to satisfy a given difficulty:

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

This project is dual-licensed under `Apache License Version 2.0` or `MIT license`.
