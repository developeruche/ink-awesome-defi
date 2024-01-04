
use openbrush::traits::AccountId;
use openbrush::traits::Balance;
use crate::providers::common::errors::StakingRewardsErrors;



#[openbrush::wrapper]
pub type StakingRewardRef = dyn StakingRewardController;

#[openbrush::trait_definition]
pub trait StakingRewardController {
    // #[ink(message)]
    // fn set_fee_to(&mut self, _fee_to: AccountId) -> Result<(), StakingRewardsErrors>;
    
    #[ink(message)]
    fn total_supply(&mut self) -> Balance;
}