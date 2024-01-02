
use ink::prelude::vec::Vec;
use ink::primitives::AccountId;
use openbrush::{traits::{Storage, Balance}, contracts::{psp22::{psp22, extensions::metadata::PSP22MetadataImpl}, traits::psp22::PSP22Ref}};
use crate::providers::{data::pair::{PairStorage, get_token_0, get_token_1, get_factory, get_price0_cumulative_last, get_price1_cumulative_last}, common::errors::UniswapV2Errors};
use scale::CompactAs; 
use sp_arithmetic::FixedU128; 


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

    fn mint(&mut self, _to: AccountId) -> Result<(), UniswapV2Errors> {
        let state = *self.data::<PairStorage>();
        let caller = Self::env().caller();
        let (reserve_0, reserve_1, block_timestamp_last) = self.get_reserves();
        let balance0 = PSP22Ref::balance_of(&state.token_0, caller);
        let balance1 = PSP22Ref::balance_of(&state.token_1, caller);
        let amount0 = balance0 - reserve_0;
        let amount1 = balance1 - reserve_1;
        let liqudity: Balance;

        let feeOn = self.internal_mint_fee();


        if self._total_supply() == 0 {
            // this is the first liquidity is been added to the pool
            liqudity = internal_sqrt(amount0 * amount1) - state.minimum_liquidity;
            self._mint_to(AccountId::from([0u8;32]), state.minimum_liquidity)?;
        } else {
            liqudity = (amount0 * self._total_supply() / reserve_0).min(amount1 * self._total_supply() / reserve_1);
        }

        if liqudity == 0 {
            return Err(UniswapV2Errors::InsufficientLiquidityMinted);
        }

        self._mint_to(_to, liqudity)?;

        
        // let liquidity = if reserve_0 == 0 && reserve_1 == 0 {
        //     amount0 * amount1
        // } else {
        //     amount0.min(amount1) * 1000 / amount0.max(amount1)
        // };
        // if liquidity == 0 {
        //     return Err(UniswapV2Errors::PSP22Error(psp22::PSP22Error::InsufficientBalance));
        // }
        // let mint_liquidity = liquidity * 1000 / 997;
        // let mint_amount0 = amount0 * mint_liquidity / liquidity;
        // let mint_amount1 = amount1 * mint_liquidity / liquidity;
        // if mint_amount0 == 0 || mint_amount1 == 0 {
        //     return Err(UniswapV2Errors::PSP22Error(psp22::PSP22Error::InsufficientBalance));
        // }
        // self.internal_transfer(state.token_0, _to, mint_amount0);
        // self.internal_transfer(state.token_1, _to, mint_amount1);
        // state.reserve_0 += mint_amount0;
        // state.reserve_1 += mint_amount1;
        // state.block_timestamp_last = block_timestamp_last;
        Ok(())
    }







    // =========================================
    // Internal Functions 
    // =========================================
    fn internal_transfer(&mut self, _token: AccountId, _to: AccountId, _value: u128) -> bool {
        return PSP22Ref::transfer(&_token, _to, _value, Vec::<u8>::new()).is_ok();
    }

    fn internal_mint_fee(&mut self) -> bool {
        let state = self.data::<PairStorage>();
        let fee_to = Some(AccountId::from([1u8; 32])); // PSP22Ref::fee_to(&state.token_0); //TODO: fee_to

        let return_data = fee_to.is_some();

        if return_data {
            if state.k_last != 0 {
                let root_k_pre = (state.reserve_0 as u128) * (state.reserve_1 as u128);
                let root_k = internal_sqrt(root_k_pre);
                let root_k_last = internal_sqrt(state.k_last);

                if root_k > root_k_last {
                    let numerator = self._total_supply() * (root_k - root_k_last);
                    let denominator = (root_k * 5) + root_k_last;
                    let liquidity = numerator / denominator;
                    if liquidity > 0 {
                        self._mint_to(fee_to.unwrap(), liquidity);
                    }
                }
            }
        }
        return_data
    }




}



fn internal_sqrt(_y: u128) -> u128 {
    let d1 = FixedU128::from_inner(_y);  
    let d2 = FixedU128::sqrt(d1); 
    let d3 = *d2.encode_as();
    d3 
}