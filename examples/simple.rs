/* The easiest way to use this crate is with the default configuration.
 * See `Default` implementation for the default configuration.
 */

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
