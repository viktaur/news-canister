use chrono::{DateTime, Utc};
use candid::Principal;

pub struct Statement {
    content: String
}

// TODO replace by the proper state manager in ICP
pub struct State;


pub struct ProofInput {
    x: Statement,
    w: Article,
    state: State,
}

pub struct Article {
    title: String,
    date: DateTime<Utc>,
    author: Principal,
    content: String
}
