use risc0_zkvm::guest::env;
use silent_whistle_core::ProofInput;

fn main() {
    // read the input
    let input: ProofInput = env::read();

    // Verify that the identity is accepted
    if !input.state.contains(input.article.author) {
        panic!("Article's author is not among the elite authorities.")
    }

    // Check that the statement is a substring of the original document
    if !input.article.content.contains(input.x) {
        panic!("Article does not contain specified fragment.");
    }

    // write public output to the journal
    env::commit();
}
