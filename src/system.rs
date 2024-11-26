use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap;

pub trait Config {
	type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config>  {
    block_number : T::BlockNumber,
    nonce : BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number : T::BlockNumber::zero(),
            nonce : BTreeMap::new(),
        }
    }
    //A function to get the current block number
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    //function used to implement the block number by one
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    //A function to keep track of how many transaction an account has made
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let current_nonce: T::Nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        let new_nonce = current_nonce + T::Nonce::one();
        self.nonce.insert(who.clone(), new_nonce); 
    }

    // A function to retrieve the nonce of a given account
    pub fn get_nonce(&self, who: &T::AccountId) -> Option<&T::Nonce> {
        self.nonce.get(who)
    }
    
}