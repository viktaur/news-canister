use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
use silent_whistle_core::{Article, Proof, State, Statement, ProofInput};
use std::{borrow::Borrow, cell::RefCell, thread};

pub mod zk;

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// #[ic_cdk::update]
// fn prove_statement(statement: Statement, article: Article) -> Proof {
//     STATE.with(|state| {
//         let proof_input = ProofInput {
//             x: statement.content,
//             w: article,
//             state: state.clone().into_inner()
//         };
//         crate::zk::prove(proof_input)
//     })
// }

// #[ic_cdk::query]
// fn verify_fact(proof: Proof) -> bool {
//     // crate::zk::verify(proof)
//     todo!()
// }
