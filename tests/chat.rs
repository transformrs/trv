mod common;

use common::load_key;
use common::trf;
use predicates::prelude::*;
use transformrs::Provider;

fn canonicalize_response(text: &str) -> String {
    text.to_lowercase()
        .trim()
        .trim_end_matches('.')
        .trim_end_matches('!')
        .to_string()
}

#[test]
fn unexpected_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = trf();
    cmd.arg("foobar");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));

    Ok(())
}
