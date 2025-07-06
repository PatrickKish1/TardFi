use risc0_zkvm::guest::env;
use oil_tokenization_core::{ OverallState, OverallParams };

fn main() {
    // Read the input from the host
    let params: OverallParams = env::read();

    // Initialize the overall state
    let overall_state = OverallState::init(
        params.oil_token_leaves,
        params.tokenization_leaves,
        params.comment_leaves
    );

    // Write the state to the journal
    env::commit(&overall_state);
}
