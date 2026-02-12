#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec};

#[contract]
pub struct GabeContract;

#[contractimpl]
impl GabeContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut vec = Vec::new(&env);
        vec.push_back(Symbol::new(&env, "Hello"));
        vec.push_back(to);
        vec
    }
}
