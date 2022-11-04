#![no_std]

use soroban_sdk::{contractimpl, BytesN, Env};

mod error_contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_errors_contract.wasm"
    );
}

pub struct Consumer;

#[contractimpl]
impl Consumer {
    pub fn call_inc(env: Env, contract_id: BytesN<32>, x: u32, y: u32) {
        let client = error_contract::Client::new(&env, contract_id);
        client.increment();
    }
}