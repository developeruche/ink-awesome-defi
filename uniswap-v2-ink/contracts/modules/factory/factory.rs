#![cfg_attr(not(feature = "std"), no_std, no_main)]



#[ink::contract]
mod factory {
    use global::providers::common::errors::UniswapV2Errors;
    use global::providers::data::factory::FactoryStorage;
    // use global::controllers::factory::factorycontroller_external;
    // use global::controllers::factory::FactoryController;
    use global::controllers::factory::factorycontroller_external::FactoryController;
    use global::providers::deployables::factory::FactoryImpl;
    use openbrush::traits::Storage;


   
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Factory {
        #[storage_field]
        pub factory_state: FactoryStorage,

    }

    impl  FactoryImpl for Factory {}


    impl FactoryController for Factory {
        #[ink(message)]
        fn set_fee_to(&mut self, _fee_to: AccountId) -> Result<(), UniswapV2Errors> {
            FactoryImpl::set_fee_to(self, _fee_to)
        }

        #[ink(message)]
        fn set_fee_to_setter(&mut self, _fee_to_setter: AccountId) -> Result<(), UniswapV2Errors> {
            FactoryImpl::set_fee_to_setter(self, _fee_to_setter)
        }

        #[ink(message)]
        fn set_pair_code_hash(&mut self, _pair_code_hash: [u8; 32]) -> Result<(), UniswapV2Errors> {
            FactoryImpl::set_pair_code_hash(self, _pair_code_hash)
        }
    }

    impl Factory {
      
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default() 
        }
      
      
    }

}