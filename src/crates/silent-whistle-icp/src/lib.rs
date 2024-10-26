use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
use risc0_zkvm::Receipt;

pub mod zk;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
fn prove_fact(fact: Fact) -> Proof {
    crate::zk::prove(fact)
}

#[ic_cdk::query]
fn verify_fact(proof: Proof) -> bool {
    crate::zk::verify(proof)
}

#[derive(CandidType)]
pub struct Proof {
    receipt: Receipt,
}

impl Proof {
    fn from(receipt: Receipt) -> Self {
        Proof {
            receipt
        }
    }
}

#[derive(Serialize, Deserialize, CandidType)]
struct Article {
    title: String,
    date: u64,
    content: String,
    publisher: Publisher,
    url: String,
}

#[derive(Serialize, Deserialize, CandidType)]
struct Fact<'a> {
    content: &'a str,
    article: &Article
}

#[derive(Serialize, Deserialize)]
struct Publisher {
    name: String,
    identity: Principal,
}
