use risc0_zkvm::guest::env;
use oil_tokenization_core::TokenizationState;

fn main() {
    // Read the input from the host
    let tokenization_leaves: Vec<String> = env::read();

    // Initialize the tokenization state
    let tokenization_state = TokenizationState::init(tokenization_leaves);

    // Write the state to the journal
    env::commit(&tokenization_state);
}
