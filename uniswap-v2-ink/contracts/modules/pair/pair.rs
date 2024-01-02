#![cfg_attr(not(feature = "std"), no_std, no_main)]



#[openbrush::implementation(PSP22, PSP22Permit, Nonces, Ownable, PSP22Metadata)]
#[openbrush::contract]
mod pair {

    use global::providers::common::errors::UniswapV2Errors;
    use global::providers::data::pair::set_factory;
    use global::providers::{data::pair::PairStorage, deployables::pair::PairImpl};
    use global::controllers::pair::PairController;
    use openbrush::{traits::Storage, contracts::reentrancy_guard};
    use global::controllers::pair::paircontroller_external;



    // =========================================
    // Storage
    // =========================================
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Pair {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        nonces: nonces::Data,
        #[storage_field]
        psp22_permit: psp22::extensions::permit::Data,
        #[storage_field]
        pub guard: reentrancy_guard::Data,
        #[storage_field]
        pub pool_state: PairStorage,
    }


    // =========================================
    // Overriding PSP22 Functions
    // =========================================




    // =========================================
    // Events 
    // =========================================
    #[ink(event)]
    pub struct Mint {
        #[ink(topic)]
        pub sender: AccountId,
        pub amount1: Balance,
        pub amount2: Balance,
    }

    #[ink(event)]
    pub struct Burn {
        #[ink(topic)]
        pub sender: AccountId,
        pub amount1: Balance,
        pub amount2: Balance,
        #[ink(topic)]
        pub to: AccountId,
    }

    #[ink(event)]
    pub struct Swap {
        #[ink(topic)]
        pub sender: AccountId,
        pub amount0_in: Balance,
        pub amount1_in: Balance,
        pub amount0_out: Balance,
        pub amount1_out: Balance,
        #[ink(topic)]
        pub to: AccountId,
    }

    #[ink(event)]
    pub struct Sync {
        pub reserve0: Balance,
        pub reserve1: Balance,
    }




    impl  PairImpl for Pair {}





    impl PairController for Pair {
        // =========================================
        // View Functions
        // =========================================
        #[ink(message)]
        fn token_0(&self) -> AccountId {
            PairImpl::token_0(self)
        }

        #[ink(message)]
        fn token_1(&self) -> AccountId {
            PairImpl::token_1(self)
        }

        #[ink(message)]
        fn factory(&self) -> AccountId {
            PairImpl::factory(self)
        }

        #[ink(message)]
        fn price0_cumulative_last(&self) -> u128 {
            PairImpl::price0_cumulative_last(self)
        }

        #[ink(message)]
        fn price1_cumulative_last(&self) -> u128 {
            PairImpl::price1_cumulative_last(self)
        }

        #[ink(message)]
        fn get_reserves(&self) -> (u128, u128, u128) {
            PairImpl::get_reserves(self)
        }

        


        // =========================================
        // Write Functions
        // =========================================
        #[ink(message)]
        fn initialize(&mut self, _token0: AccountId, _token1: AccountId) {
           PairImpl::initialize(self, _token0, _token1);
        }

        #[ink(message)]
        fn mint(&mut self, _to: AccountId) -> Result<(), UniswapV2Errors> {
           PairImpl::mint(self, _to)
        }

        #[ink(message)]
        fn burn(&mut self, _to: AccountId) -> Result<(), UniswapV2Errors> {
           PairImpl::burn(self, _to)
        }

        #[ink(message)]
        fn swap(&mut self, _amount0: Balance, _amount1: Balance, _to: AccountId)  -> Result<(), UniswapV2Errors> {
           PairImpl::swap(self, _amount0, _amount1, _to)
        }

        #[ink(message)]
        fn skim(&mut self, _to: AccountId)  -> Result<(), UniswapV2Errors> {
           PairImpl::skim(self, _to)
        }

        #[ink(message)]
        fn sync(&mut self) -> Result<(), UniswapV2Errors> {
           PairImpl::sync(self)
        }
    }


    impl Pair {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8, factory: AccountId) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();

            instance.metadata.name.set(&name);
            instance.metadata.symbol.set(&symbol);
            instance.metadata.decimals.set(&decimal);

            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            psp22::Internal::_mint_to(&mut instance, caller, total_supply).expect("Should mint total_supply");

            set_factory(&mut instance, factory);
            
            instance
        }
    }

}