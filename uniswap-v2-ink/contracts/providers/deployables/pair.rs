
use ink::primitives::AccountId;
use openbrush::{traits::Storage, contracts::psp22::{psp22, extensions::metadata::PSP22MetadataImpl}};
use crate::providers::data::pair::PairStorage;

pub trait PairImpl:  
    Storage<PairStorage> +
    Storage<psp22::Data> +
    PSP22MetadataImpl +
    // psp22::Internal + 
{
    fn initialize(&mut self,  _token0: AccountId, _token1: AccountId) {
    
        let state = self.data::<PairStorage>();
        state.token_0 = _token0;
        state.token_1 = _token1;
    }
}