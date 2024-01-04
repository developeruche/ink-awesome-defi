#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[openbrush::implementation(Ownable)]
#[openbrush::contract]
mod staking_reward {
    use ink::prelude::vec::Vec;
    use global::providers::common::errors::StakingRewardsErrors;
    use global::providers::data::staking_reward::StakingStorage;
    use global::controllers::staking_reward::stakingrewardcontroller_external::StakingRewardController;
    use global::providers::deployables::staking_reward::StakingRewardImpl;
    use openbrush::contracts::ownable;
    use openbrush::traits::Storage;
    use global::controllers::staking_reward::stakingrewardcontroller_external;
   


    // ======================================
    // Storage
    // ======================================
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakingReward {
        #[storage_field]
        pub staking_state: StakingStorage,
        #[storage_field]
        ownable: ownable::Data,
    }



    // ======================================
    // Events
    // ======================================
    #[ink(event)]
    pub struct Staked {
        #[ink(topic)]
        caller:AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Withdraw {
        #[ink(topic)]
        caller:AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct RewardPaid {
        #[ink(topic)]
        caller:AccountId,
        reward: Balance,
    }

    #[ink(event)]
    pub struct RewardNotified {
        reward: Balance,
    }

    #[ink(event)]
    pub struct DurationUpdate {
        duration: Balance,
    }




    impl  StakingRewardImpl for StakingReward {}


    // impl StakingRewardController for StakingReward {
    //     #[ink(message)]
    //     fn total_supply(&mut self) -> Balance {
    //         StakingRewardImpl::total_supply(self)
    //     }
    // }

    impl StakingReward {
      
        #[ink(constructor)]
        pub fn new(_owner: AccountId, _reward_token: AccountId, _staked_token: AccountId) -> Self {
            let mut instance = Self::default();


            ownable::Internal::_init_with_owner(&mut instance, _owner);
            instance.staking_state.staked_token = _staked_token;
            instance.staking_state.reward_token = _reward_token;


            instance
        }




        #[ink(message)]
        pub fn create_pair(&mut self, token_a: AccountId, token_b: AccountId, salt_bytes: Vec<u8>) -> Result<AccountId, StakingRewardsErrors> {
            todo!()
        }
      
      
    }

}