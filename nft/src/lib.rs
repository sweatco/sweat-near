use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue};

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example SWEAT nft".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();

        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    #[payable]
    pub fn nft_mint(&mut self, token_id: TokenId, receiver_id: AccountId, token_metadata: TokenMetadata) -> Token {
        self.tokens.internal_mint(token_id, receiver_id, Some(token_metadata))
    }

    #[payable]
    pub fn batch_nft_mint(&mut self) {
        let now = env::block_timestamp_ms();
        for n in 0..10 {
            self.nft_mint(
                format!("batch_token_{}_{}", now, n),
                AccountId::new_unchecked("sweat_the_token.testnet".to_string()),
                TokenMetadata {
                    title: Some(format!("batch_token_{}_{}", now, n)),
                    description: Some("really super".to_string()),
                    media: Some("https://assets.untappd.com/site/beer_logos_hd/beer-1988545_8fe4a_hd.jpeg".to_string()),
                    media_hash: None,
                    copies: Some(1),
                    issued_at: None,
                    expires_at: None,
                    starts_at: None,
                    updated_at: None,
                    extra: None,
                    reference: None,
                    reference_hash: None,
                },
            );
        }
    }

    #[payable]
    pub fn mint_dummy(&mut self, token_id: TokenId, receiver_id: AccountId) {
        self.nft_mint(
            token_id.clone(),
            receiver_id.clone(),
            TokenMetadata {
                title: Some(format!("Test mutable NFT {}", token_id.clone())),
                description: Some("really super".to_string()),
                media: Some("https://assets.untappd.com/site/beer_logos_hd/beer-1988545_8fe4a_hd.jpeg".to_string()),
                media_hash: None,
                copies: Some(1),
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            },
        );
    }

    pub fn update_extra(&mut self, token_id: TokenId, extra: String) {
        let lookup_map = self.tokens.token_metadata_by_id.as_mut().unwrap();
        let mut metadata = lookup_map.get(&token_id.to_string()).unwrap();
        metadata.extra = Some(extra);
        lookup_map.insert(&token_id, &metadata);
    }

    pub fn update_media(&mut self, token_id: TokenId, media: String) {
        let lookup_map = self.tokens.token_metadata_by_id.as_mut().unwrap();
        let mut metadata = lookup_map.get(&token_id.to_string()).unwrap();
        metadata.media = Some(media);
        lookup_map.insert(&token_id, &metadata);
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
