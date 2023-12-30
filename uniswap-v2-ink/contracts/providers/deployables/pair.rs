
use openbrush::{traits::Storage, contracts::psp22::{psp22, extensions::metadata::PSP22MetadataImpl}};
use crate::providers::data::pair::PairStorage;

pub trait PairImpl:  
    Storage<PairStorage> +
    Storage<psp22::Data> +
    PSP22MetadataImpl +
    psp22::Internal + 
{

        // fn flip(&mut self) {
            
        //     let mut state = *self.data::<Contract2Storage>();
        //     state.value = !state.value;
        // }

        // fn get(&self) -> bool {

        //     let state = *self.data::<Contract2Storage>();
        //     state.value
        // }
}