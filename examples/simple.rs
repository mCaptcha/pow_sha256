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

    const DIFFICULTY: u32 = 1000;

    let work = config.prove_work(&phrase, DIFFICULTY).unwrap();
    assert!(config.is_valid_proof(&work, &phrase));
    assert!(config.is_sufficient_difficulty(&work, DIFFICULTY));
}
