#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env
};

use soroban_sdk::token::Client as TokenClient;

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

    pub fn transfer_with_payment(
        env: Env,
        token_id: u32,
        buyer: Address,
        token: Address,
        sale_price: i128,
    ) {
        let mut nft: NFT = env
            .storage()
            .instance()
            .get(&token_id)
            .expect("NFT not found");

        // Seller must authorize
        nft.owner.require_auth();

        let seller = nft.owner.clone();
        let creator = nft.creator.clone();

        // Royalty calculation (basis points)
        let royalty = sale_price * (nft.royalty_bps as i128) / 10_000;
        let seller_amount = sale_price - royalty;

        let token_client = TokenClient::new(&env, &token);

        let contract_address = env.current_contract_address();

        token_client.transfer_from(
            &contract_address, // spender (the NFT contract itself)
            &buyer,            // from
            &creator,          // to
            &royalty,
        );

        token_client.transfer_from(
            &contract_address,
            &buyer,
            &seller,
            &seller_amount,
        );

        // Update ownership
        nft.owner = buyer;

        env.storage().instance().set(&token_id, &nft);
    }
}
