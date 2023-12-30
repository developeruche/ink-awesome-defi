#![cfg_attr(not(feature = "std"), no_std, no_main)]



#[openbrush::implementation(PSP22, PSP22Permit, Nonces, PSP22Mintable, Ownable, PSP22Metadata)]
#[openbrush::contract]
mod factory {


    use global::providers::{data::pair::PairStorage, deployables::pair::PairImpl};
    // use global::providers::{data::contract_2::Contract2Storage, deployables::contract_2::Contract2Impl};
    // use global::controllers::contract_2::contract2controller_external::Contract2Controller;
    use openbrush::{traits::Storage, modifiers, contracts::reentrancy_guard};



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

    #[default_impl(PSP22Mintable)]
    #[modifiers(only_owner)]
    fn mint(&mut self) {}


    impl  PairImpl for Pair {}


    impl Contract2Controller for Contract2 {
        #[ink(message)]
        fn flip(&mut self) {
           Contract2Impl::flip(self);
        }

        #[ink(message)]
        fn get(&self) -> bool {
            Contract2Impl::get(self)
        }
    }

    impl Pair {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();

            instance.metadata.name.set(&name);
            instance.metadata.symbol.set(&symbol);
            instance.metadata.decimals.set(&decimal);

            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            psp22::Internal::_mint_to(&mut instance, caller, total_supply).expect("Should mint total_supply");
            

            instance
        }



        #[ink(message)]
        pub fn burn(&mut self, _amount: Balance) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            psp22::Internal::_burn_from(self, caller, _amount)
        }
    }

}