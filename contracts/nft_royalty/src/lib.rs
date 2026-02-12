#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env
};

#[derive(Clone)]
#[contracttype]
pub struct NFT {
    pub owner: Address,
    pub creator: Address,
    pub royalty_bps: u32,
}

#[contract]
pub struct NftRoyaltyContract;

#[contractimpl]
impl NftRoyaltyContract {

    // Mint a new NFT
    pub fn mint(
        env: Env,
        token_id: u32,
        creator: Address,
        royalty_bps: u32,
    ) {
        creator.require_auth();

        let nft = NFT {
            owner: creator.clone(),
            creator,
            royalty_bps,
        };

        env.storage().instance().set(&token_id, &nft);
    }

    // Simple transfer (no payment logic yet)
    pub fn transfer(
        env: Env,
        token_id: u32,
        new_owner: Address,
    ) {
        let mut nft: NFT = env
            .storage()
            .instance()
            .get(&token_id)
            .expect("NFT not found");

        nft.owner.require_auth();

        nft.owner = new_owner;

        env.storage().instance().set(&token_id, &nft);
    }
}
