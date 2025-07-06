use risc0_zkvm::guest::env;
use oil_tokenization_core::CommentState;

fn main() {
    // Read the input from the host
    let comment_leaves: Vec<String> = env::read();

    // Initialize the comment state
    let comment_state = CommentState::init(comment_leaves);

    // Write the state to the journal
    env::commit(&comment_state);
}
