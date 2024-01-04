#![cfg_attr(not(feature = "std"), no_std, no_main)]



#[ink::contract]
mod staking_reward {
    use ink::prelude::vec::Vec;
    use global::providers::common::errors::StakingRewardsErrors;
    use global::providers::data::staking_reward::StakingStorage;
    use global::controllers::staking_reward::stakingrewardcontroller_external::StakingRewardController;
    use global::providers::deployables::staking_reward::StakingRewardImpl;
    use openbrush::traits::Storage;
   


    // ======================================
    // Storage
    // ======================================
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakingReward {
        #[storage_field]
        pub staking_state: StakingStorage,

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


    impl StakingRewardController for StakingReward {
        #[ink(message)]
        fn total_supply(&mut self) -> Balance {
            StakingRewardImpl::total_supply(self)
        }
    }

    impl StakingReward {
      
        #[ink(constructor)]
        pub fn new(pair_code_hash: [u8; 32]) -> Self {
            let mut instance = Self::default();

            // instance.factory_state.fee_to_setter = Some(Self::env().caller());
            // instance.factory_state.pair_code_hash = pair_code_hash;


            instance
        }




        #[ink(message)]
        pub fn create_pair(&mut self, token_a: AccountId, token_b: AccountId, salt_bytes: Vec<u8>) -> Result<AccountId, StakingRewardsErrors> {
            todo!()
        }
      
      
    }

}