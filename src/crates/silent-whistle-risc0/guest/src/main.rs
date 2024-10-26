use risc0_zkvm::guest::env;
use news_canister_core::ProofInput;

/// Proves the given statement by looking at the identity of the publisher and comparing
/// it with the official records.
fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: ProofInput = env::read();

    // Check if Identity belongs to the set of valid identities

    // Check if statement is substring of Article content
    input.x

    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}
