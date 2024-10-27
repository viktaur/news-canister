use candid::{CandidType, Principal};
use risc0_zkvm::Receipt;
use serde::{Serialize, Deserialize};
use std::cell::Ref;
use std::collections::HashSet;
use std::thread::LocalKey;

#[derive(CandidType)]
pub struct Statement {
    pub content: String
}

// TODO replace by the proper state manager in ICP
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct State {
    articles: Vec<Article>,
    approved_ids: Vec<Principal>,
}

#[derive(Serialize, Deserialize)]
pub struct Proof {
    pub receipt: Receipt
}

#[derive(Serialize, Deserialize)]
pub struct ProofInput {
    pub x: String,
    pub w: Article,
    pub state: State,
}

#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct Article {
    pub title: String,
    pub date: u64,
    pub author: Principal,
    pub content: String
}
