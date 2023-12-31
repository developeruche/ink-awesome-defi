use ink::primitives::AccountId;

#[openbrush::wrapper]
pub type PairRef = dyn PairController ;

#[openbrush::trait_definition]
pub trait PairController {
    
    #[ink(message)]
    fn initialize(&mut self, _token0: AccountId, _token1: AccountId);
}