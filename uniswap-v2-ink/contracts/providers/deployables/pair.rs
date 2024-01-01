
use ink::{ prelude::vec::Vec };
use ink::primitives::AccountId;
use openbrush::{traits::Storage, contracts::{psp22::{psp22, extensions::metadata::PSP22MetadataImpl}, traits::psp22::PSP22Ref}};
use crate::providers::{data::pair::{PairStorage, get_token_0, get_token_1, get_factory, get_price0_cumulative_last, get_price1_cumulative_last}, common::errors::UniswapV2Errors};

pub trait PairImpl:  
    Storage<PairStorage> +
    Storage<psp22::Data> +
    PSP22MetadataImpl +
    psp22::Internal + 
{
    // public varible get functions
    fn token_0(&self) -> AccountId {
        get_token_0(self)
    }

    fn token_1(&self) -> AccountId {
        get_token_1(self)
    }

    fn factory(&self) -> AccountId {
        get_factory(self)
    }

    fn price0_cumulative_last(&self) -> u128 {
        get_price0_cumulative_last(self)
    }

    fn price1_cumulative_last(&self) -> u128 {
        get_price1_cumulative_last(self)
    }

    fn get_reserves(&self) -> (u128, u128, u128) {
        let state = self.data::<PairStorage>();
        (state.reserve_0, state.reserve_1, state.block_timestamp_last)
    }







    
    fn initialize(&mut self,  _token0: AccountId, _token1: AccountId) {
        let state = self.data::<PairStorage>();
        state.token_0 = _token0;
        state.token_1 = _token1;
    }







    // =========================================
    // Internal Functions 
    // =========================================
    fn internal_transfer(&mut self, _token: AccountId, _to: AccountId, _value: u128) -> bool {
        return PSP22Ref::transfer(&_token, _to, _value, Vec::<u8>::new()).is_ok();
    }

    
}