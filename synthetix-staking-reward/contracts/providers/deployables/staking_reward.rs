use openbrush::{traits::{Storage, AccountId}, contracts::ownable::OwnableError};
use crate::providers::{data::staking_reward::StakingStorage, common::errors::StakingRewardsErrors}; 
use openbrush::traits::Balance;


pub trait StakingRewardImpl:  
    Storage<StakingStorage> 
{
    // ======================================
    // Mutating functions
    // ======================================
    // fn set_fee_to(&mut self, _fee_to: AccountId) -> Result<(), StakingRewardsErrors> {
    //     let state = self.data::<StakingStorage>();

    //     if state.fee_to_setter.unwrap() != Self::env().caller() {
    //         return Err(StakingRewardsErrors::OwnableError(OwnableError::CallerIsNotOwner))
    //     }
    //     self.data::<StakingStorage>().fee_to_setter = Some(_fee_to);
    //     Ok(())
    // }











    // ======================================
    // View functions
    // ======================================
    fn total_supply(&self) -> Balance {
        self.data::<StakingStorage>().total_supply
    }
}