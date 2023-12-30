use openbrush::{ traits::{ AccountId, Balance, Storage } };
use ink::{ storage::{ Mapping, traits::StorageLayout } };
// use crate::providers::common::database::*;

#[derive(Debug)]
#[openbrush::storage_item(FACTORY_STORAGE_LOCATION)]
pub struct FactoryStorage {
    pub minimum_liquidity: Balance,
    pub factory: AccountId,
    pub token_0: AccountId,
    pub token_1: AccountId,
    pub reserve_0: Balance,
    pub reserve_1: Balance,
    pub block_timestamp_last: Balance,
    pub price0_cumulative_last: Balance,
    pub price1_cumulative_last: Balance,
    pub k_last: Balance,
}


impl  Default for FactoryStorage {
    fn default() -> Self {
        Self { 
            minimum_liquidity: Balance::default(),
            factory: AccountId::from([0u8; 32]),
            token_0: AccountId::from([0u8; 32]),
            token_1: AccountId::from([0u8; 32]),
            reserve_0: Balance::default(),
            reserve_1: Balance::default(),
            block_timestamp_last: Balance::default(),
            price0_cumulative_last: Balance::default(),
            price1_cumulative_last: Balance::default(),
            k_last: Balance::default(),
        }
    }
}