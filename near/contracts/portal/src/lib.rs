//#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, Gas, PanicOnDefault, Promise,
    PromiseOrValue, PromiseResult,
};

use near_sdk::serde_json::Value;

use hex;

pub mod byte_utils;
pub mod state;

use crate::byte_utils::ByteUtils;

// near_sdk::setup_alloc!();

const CHAIN_ID_NEAR: u16 = 15;
const CHAIN_ID_SOL: u16 = 1;

#[ext_contract(ext_core_bridge)]
pub trait CoreBridge {
    fn verify_vaa(&self, vaa: String) -> (String, i32);
    fn publish_message(&self, data: Vec<u8>) -> u64;
}

#[ext_contract(ext_self)]
pub trait TokenBridgeCallback {
    fn submit_vaa_callback(&mut self);
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct FTContract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

//            FungibleTokenMetadata {
//                spec: FT_METADATA_SPEC.to_string(),
//                name: "Example NEAR fungible token".to_string(),
//                symbol: "EXAMPLE".to_string(),
//                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
//                reference: None,
//                reference_hash: None,
//                decimals: 24,
//            },

impl FTContract {
    pub fn new(owner_id: AccountId, metadata: FungibleTokenMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this
    }

    fn on_mint(&mut self, dest: AccountId, amt: U128) {
        self.token.internal_deposit(&dest, amt.into());
        self.token.internal_register_account(&dest);
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &dest,
            amount: &amt,
            memo: Some("newly minted"),
        }
        .emit();
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        env::panic_str("an_account_closed");
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        env::panic_str("on_tokens_burned");
    }
}

near_contract_standards::impl_fungible_token_core!(FTContract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(FTContract, token, on_account_closed);

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenBridge {
    dups: UnorderedSet<Vec<u8>>,
    contracts: LookupMap<u16, Vec<u8>>,
    tokens: LookupMap<String, FTContract>,
    booted: bool,
    core: AccountId,
}

impl Default for TokenBridge {
    fn default() -> Self {
        Self {
            dups: UnorderedSet::new(b"d".to_vec()),
            contracts: LookupMap::new(b"c".to_vec()),
            tokens: LookupMap::new(b"t".to_vec()),
            booted: false,
            core: AccountId::new_unchecked("".to_string()),
        }
    }
}

fn vaa_register_chain(storage: &mut TokenBridge, vaa: state::ParsedVAA) {
    let data: &[u8] = &vaa.payload;
    let chain = data.get_u16(33);

    if (chain != CHAIN_ID_NEAR) && (chain != 0) {
        env::panic_str("InvalidRegisterChainChain");
    }

    if storage.contracts.contains_key(&chain) {
        env::panic_str("DuplicateChainRegistration");
    }

    storage.contracts.insert(&chain, &data[34..66].to_vec());
}

fn vaa_upgrade_contract(storage: &mut TokenBridge, vaa: state::ParsedVAA) {
    let data: &[u8] = &vaa.payload;
    let chain = data.get_u16(33);
    if chain != CHAIN_ID_NEAR {
        env::panic_str("InvalidContractUpgradeChain");
    }

    env::panic_str("ContractUpgradesNotImplemented");
}

fn hdl_governance(storage: &mut TokenBridge, vaa: state::ParsedVAA, gov_idx: u32) {
    if gov_idx != vaa.guardian_set_index {
        env::panic_str("InvalidGovernanceSet");
    }

    if (CHAIN_ID_SOL != vaa.emitter_chain)
        || (hex::decode("0000000000000000000000000000000000000000000000000000000000000004")
            .unwrap()
            != vaa.emitter_address)
    {
        env::panic_str("InvalidGovernanceEmitter");
    }

    let data: &[u8] = &vaa.payload;
    let action = data.get_u8(32);

    match action {
        1u8 => vaa_register_chain(storage, vaa),
        2u8 => vaa_upgrade_contract(storage, vaa),
        _ => env::panic_str("InvalidGovernanceAction"),
    }
}

fn vaa_transfer(storage: &mut TokenBridge, vaa: state::ParsedVAA) {
    env::panic_str("vaa_transfer");
}
fn vaa_asset_meta(storage: &mut TokenBridge, vaa: state::ParsedVAA) {
    env::panic_str("vaa_asset_meta");
}
fn vaa_transfer_with_payload(storage: &mut TokenBridge, vaa: state::ParsedVAA) {
    env::panic_str("vaa_transfer_with_payload");
}

#[near_bindgen]
impl TokenBridge {
    pub fn submit_vaa(&mut self, vaa: String) -> Promise {
        ext_core_bridge::verify_vaa(
            vaa,
            self.core.clone(),        // contract account id
            0,                        // yocto NEAR to attach
            Gas(100_000_000_000_000), // gas to attach
        )
        .then(ext_self::submit_vaa_callback(
            env::current_account_id(), // me
            0,                         // yocto NEAR to attach to the callback
            Gas(100_000_000_000_000),  // gas to attach
        ))
    }

    #[private] // So, all of wormhole security rests in this one statement?
    pub fn submit_vaa_callback(&mut self) {
        // well, and this one...
        if (env::promise_results_count() != 1)
            || (env::predecessor_account_id() != env::current_account_id())
        {
            env::panic_str("BadPredecessorAccount");
        }

        let data: String;
        match env::promise_result(0) {
            PromiseResult::Successful(result) => {
                data = String::from_utf8(result).unwrap();
            }
            _ => env::panic_str("vaaVerifyFail"),
        }

        let v: Value = near_sdk::serde_json::from_str(&data).unwrap();

        // Please, what is the correct way of just getting a fricken string?!
        let _vaa = v[0].to_string();
        let vaa = &_vaa[1.._vaa.len() - 1];

        let gov_idx = v[1].as_i64().unwrap() as u32;

        let h = hex::decode(vaa).expect("invalidVaa");

        let vaa = state::ParsedVAA::parse(&h);

        if vaa.version != 1 {
            env::panic_str("InvalidVersion");
        }

        // Check if VAA with this hash was already accepted
        if self.dups.contains(&vaa.hash) {
            env::panic_str("alreadyExecuted");
        }
        self.dups.insert(&vaa.hash);

        let data: &[u8] = &vaa.payload;

        if data[0..32]
            == hex::decode("000000000000000000000000000000000000000000546f6b656e427269646765")
                .unwrap()
        {
            hdl_governance(self, vaa, gov_idx);
            return;
        }

        let action = data.get_u8(0);

        match action {
            1u8 => vaa_transfer(self, vaa),
            2u8 => vaa_asset_meta(self, vaa),
            3u8 => vaa_transfer_with_payload(self, vaa),
            _ => env::panic_str("InvalidPortalAction"),
        }
    }

    pub fn boot_portal(&mut self, core: String) {
        if self.booted {
            env::panic_str("no donut");
        }
        self.booted = true;
        self.core = AccountId::try_from(core.clone()).unwrap();
    }
}
