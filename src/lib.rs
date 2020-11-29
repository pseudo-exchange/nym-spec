use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance, BlockHeight, env, near_bindgen};
use near_sdk::collections::UnorderedMap;
mod util;

// TODO:
// - Auction item/struct - needs account owner, time to close, bids & bidders, asset
// - new auction item
// - view auction item by ID, all
// - close/withdrawal auction - needs to be called by one of bidders or owner, release asset to highest bidder, return rewards to owner
// - cancel

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Auction<T> {
    pub owner_id: AccountId, // near account
    pub winner_account_id: Option<AccountId>,
    pub asset: T,
    pub close_block: BlockHeight, // Needs checking that theres no race case transactions
    bids: UnorderedMap<AccountId, Balance>,
}

impl ToString for Auction<String> {
    fn to_string(&self) -> String {
        let fields = vec![
            self.owner_id.to_string(), 
            self.asset.to_string(), 
            self.close_block.to_string()
        ];
        fields.join("")
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AuctionHouse {
    pub auctions: UnorderedMap<String, Auction<String>>
}

impl Default for AuctionHouse {
    fn default() -> Self {
        AuctionHouse {
            auctions: UnorderedMap::new(
                env::keccak256(
                    env::block_index().to_string().as_bytes()
                )
            )
        }
    }
}

#[near_bindgen]
impl AuctionHouse {
    pub fn create(&mut self, asset: String) {
        let auction = Auction {
            asset,
            owner_id: env::signer_account_id(),
            close_block: env::block_index() + 100,
            winner_account_id: None,
            bids: UnorderedMap::new(
                env::keccak256(
                    env::block_index().to_string().as_bytes()
                )
            )
        };
        
        // Convert our auction to a string & compute the keccak256 hash
        let key = String::from_utf8(env::keccak256(
            auction.to_string().as_bytes()
        )).expect("Failed to create auction hash");

        // Error check for failed insertion
        if let None = self.auctions.insert(
            &key, 
            &auction
        ) {
            panic!("Failed to create new auction")
        }

        // Use our fancy Macro, because KA CHING!
        logger!("Created new auction {}", key);
    }

    // Becuz of a typo of view
    pub fn view(&self, id: Option<String>) -> String {
        match id {
            Some(result) => String::from_utf8(
                self.auctions.get(
                    &result
                ).try_to_vec().unwrap()
            ).unwrap(),
            None => panic!("Auction ID: {} not found", id.unwrap()),
        }
    }

    // Allow anyone to place a bid on an auction,
    // which accepts an auction id and attached_deposit balance for contribution which buys the asset
    // 
    // Requires:
    // - user to NOT be owner
    // - auction amount needs to be greater than 0
    // - auction needs to not be closed
    // 
    // Optional:
    // - user CAN update bid by calling this fn multiple times
    pub fn place_bid(&mut self, auction_id: String) {
        if let Some(auction) = self.auctions.get(&auction_id) {
            assert_ne!(
                auction.owner_id, env::signer_account_id(),
                "Must not be owner of auction"
            );
            assert!(
                env::attached_deposit() > 0,
                "Must submit bid amount of greater than zero"
            );
            assert!(
                env::block_index() > auction.close_block,
                "Must be an active auction"
            );
        } else { 
            panic!("Shit got real");
        }
    }

    // removes an auction if owner called it
    // sends back all auction bidders their funds
    pub fn cancel_auction(&mut self, auction_id: String) {
        if let Some(auction) = self.auctions.get(&auction_id) {
            assert_eq!(
                auction.owner_id, env::signer_account_id(),
                "Must be owner to cancel auction"
            );

            // TODO: Send bidders their funds

            // remove auction data
            self.auctions.remove(&auction_id);
        } else {
            panic!("Failed to cancel auction")
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Thing::default();
        assert_eq!(None, contract.set_thing("francis.near".to_string()));
    }
}
