use risc0_zkvm::guest::env;
use oil_tokenization_core::OilTokenState;

fn main() {
    // Read the input from the host
    let oil_token_leaves: Vec<String> = env::read();

    // Initialize the oil token state
    let oil_token_state = OilTokenState::init(oil_token_leaves);

    // Write the state to the journal
    env::commit(&oil_token_state);
}
