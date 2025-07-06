// #![cfg_attr(not(test), no_std)]
use serde::{ Deserialize, Serialize };
use risc0_zkp::core::digest::Digest;
use tiny_keccak::{ Hasher, Keccak };
use chrono::{ DateTime, Local };

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VerifyParams {
    pub message: String,
    pub signature_bytes: String,
    pub expected_addr: String,
    pub timestamp: i64,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VerifyCommit {
    pub verified: bool,
    pub address: String,
    pub timestamp: i64,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct OverallParams {
    pub oil_token_leaves: Vec<String>,
    pub tokenization_leaves: Vec<String>,
    pub comment_leaves: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OilTokenizationState {
    pub oil_token_state: Digest,
    pub tokenization_state: Digest,
    pub comment_state: Digest,
    pub overall: Digest,
}

// tokenization commit :: CRUD
// comment commit :: CRUD
// oil_token commit :: CRUD
pub enum OilTokenizationAction {}
impl OilTokenizationState {
    pub fn init() {
        // start of the database
        // stores necessay details to communicate with smart contract.
        // stores initial state on the blockchain
        // initial state serves as the start of the merkle tree on the blockchain (oil_token, tokenization, comment, overall)
        // cannot reset state or reinit state.
    }

    pub fn process() {
        // process results in change of state
        // update state and sync with remote state.
    }

    fn sync() {
        //sync state with smart contract
        //verify localState with remoteState (contract)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TokenizationAction {
    INIT,
    CREATE,
    UPDATE,
    DELETE,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Actor {
    ADMIN,
    SYSTEM,
}

// actor rules - clearly stating what those actors does and their permissions.
// allow for transparency on who issued commands without revealing the actors identity.

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TokenizationState {
    pub old_state: Vec<Digest>,
    pub new_state: Vec<Digest>,
    pub overall: Digest,
    pub leaves: usize,
    pub action: TokenizationAction,
    pub actor: Actor,
}

// verify actor before completing a process.
impl TokenizationState {
    pub fn init(db: Vec<String>) -> Self {
        let mut tokenization_state: Vec<Digest> = vec![];
        let mut hasher = Keccak::v256();
        let mut hasherb = Keccak::v256();
        for x in &db {
            hasher.update(&x.as_bytes());
            hasherb.update(&x.as_bytes());
            let mut output = [0; 32];
            hasher.clone().finalize(&mut output);
            let digest = Digest::from_bytes(output);
            tokenization_state.push(digest);
        }
        let mut output = [0; 32];
        hasherb.finalize(&mut output);
        let digest = Digest::from_bytes(output);

        TokenizationState {
            old_state: tokenization_state.clone(),
            new_state: tokenization_state,
            overall: digest,
            leaves: db.len(),
            action: TokenizationAction::INIT,
            actor: Actor::SYSTEM,
        }
    }

    pub fn update(&mut self, new_state: Vec<Digest>, overall: Digest) {
        self.old_state = self.new_state.clone();
        self.new_state = new_state;
        self.overall = overall;
    }

    pub fn process(action: TokenizationAction) {
        match action {
            TokenizationAction::INIT => {
                // call the function here and make sure it workes.
                // each function call returns the new state which will be used here to update the state.
            }
            _ => {}
        }
    }

    pub fn sync() {}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CommentAction {
    INIT,
    CREATE,
    UPDATE,
    DELETE,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommentState {
    pub old_state: Vec<Digest>,
    pub new_state: Vec<Digest>,
    pub overall: Digest,
    pub leaves: usize,
    pub action: CommentAction,
    pub actor: Actor,
}

impl CommentState {
    pub fn init(db: Vec<String>) -> Self {
        let mut comment_state: Vec<Digest> = vec![];
        let mut hasher = Keccak::v256();
        let mut hasherb = Keccak::v256();
        for x in &db {
            hasher.update(&x.as_bytes());
            hasherb.update(&x.as_bytes());
            let mut output = [0; 32];
            hasher.clone().finalize(&mut output);
            let digest = Digest::from_bytes(output);
            comment_state.push(digest);
        }
        let mut output = [0; 32];
        hasherb.finalize(&mut output);
        let digest = Digest::from_bytes(output);

        CommentState {
            old_state: comment_state.clone(),
            new_state: comment_state,
            overall: digest,
            leaves: db.len(),
            action: CommentAction::INIT,
            actor: Actor::SYSTEM,
        }
    }

    pub fn update(&mut self, new_state: Vec<Digest>, overall: Digest) {
        self.old_state = self.new_state.clone();
        self.new_state = new_state;
        self.overall = overall;
    }

    pub fn process(action: CommentAction) {
        match action {
            CommentAction::INIT => {
                // call the function here and make sure it workes.
                // each function call returns the new state which will be used here to update the state.
            }
            _ => {}
        }
    }

    pub fn sync() {}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OverallState {
    pub oil_token_state: OilTokenState,
    pub tokenization_state: TokenizationState,
    pub comment_state: CommentState,
    pub old_state: Digest,
    pub new_state: Digest,
    pub updated_at: String,
}

impl OverallState {
    pub fn new() -> Self {
        OverallState {
            oil_token_state: OilTokenState::init(vec![]),
            tokenization_state: TokenizationState::init(vec![]),
            comment_state: CommentState::init(vec![]),
            old_state: Digest::from_bytes([0; 32]),
            new_state: Digest::from_bytes([0; 32]),
            updated_at: Local::now().to_rfc3339(),
        }
    }

    pub fn init(
        oil_token_leaves: Vec<String>,
        tokenization_leaves: Vec<String>,
        comment_leaves: Vec<String>
    ) -> Self {
        let oil_token_state = OilTokenState::init(oil_token_leaves);
        let tokenization_state = TokenizationState::init(tokenization_leaves);
        let comment_state = CommentState::init(comment_leaves);

        let mut hasher = Keccak::v256();
        hasher.update(&oil_token_state.overall.as_bytes());
        hasher.update(&tokenization_state.overall.as_bytes());
        hasher.update(&comment_state.overall.as_bytes());
        let mut output = [0; 32];
        hasher.finalize(&mut output);
        let overall_digest = Digest::from_bytes(output);

        OverallState {
            oil_token_state,
            tokenization_state,
            comment_state,
            old_state: overall_digest,
            new_state: overall_digest,
            updated_at: Local::now().to_rfc3339(),
        }
    }

    pub fn sync(
        &self,
        oil_token: &OilTokenState,
        tokenization: &TokenizationState,
        comment: &CommentState
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut hasher = Keccak::v256();
        hasher.update(&oil_token.overall.as_bytes());
        hasher.update(&tokenization.overall.as_bytes());
        hasher.update(&comment.overall.as_bytes());
        let mut output = [0; 32];
        hasher.finalize(&mut output);
        let overall_digest = Digest::from_bytes(output);

        Ok(OverallState {
            oil_token_state: oil_token.clone(),
            tokenization_state: tokenization.clone(),
            comment_state: comment.clone(),
            old_state: self.new_state,
            new_state: overall_digest,
            updated_at: Local::now().to_rfc3339(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OilTokenAction {
    INIT,
    CREATE, // insert oil token
    UPDATE, // update oil token info
    BUY, // buy oil token
    SELL, // Sell oil token
    // DELETE, // delete oil token
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OilTokenState {
    pub old_state: Vec<Digest>,
    pub new_state: Vec<Digest>,
    pub overall: Digest,
    pub leaves: usize,
    pub action: OilTokenAction,
    pub actor: Actor,
}

impl OilTokenState {
    pub fn init(db: Vec<String>) -> Self {
        let mut oil_token_state: Vec<Digest> = vec![];
        let mut hasher = Keccak::v256();
        let mut hasherb = Keccak::v256();
        for x in &db {
            hasher.update(&x.as_bytes());
            hasherb.update(&x.as_bytes());
            let mut output = [0; 32];
            hasher.clone().finalize(&mut output);
            let digest = Digest::from_bytes(output);
            oil_token_state.push(digest);
        }
        let mut output = [0; 32];
        hasherb.finalize(&mut output);
        let digest = Digest::from_bytes(output);

        OilTokenState {
            old_state: oil_token_state.clone(),
            new_state: oil_token_state,
            overall: digest,
            leaves: db.len(),
            action: OilTokenAction::INIT,
            actor: Actor::SYSTEM,
        }
    }

    pub fn update(&mut self, new_state: Vec<Digest>, overall: Digest) {
        self.old_state = self.new_state.clone();
        self.new_state = new_state;
        self.overall = overall;
    }

    pub fn process(action: OilTokenAction) {
        match action {
            OilTokenAction::INIT => {
                // call the function here and make sure it workes.
                // each function call returns the new state which will be used here to update the state.
            }
            _ => {}
        }
    }

    pub fn sync() {}
}

// message: &[u8], signature_bytes: &[u8; 65]
