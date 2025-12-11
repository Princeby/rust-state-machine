use std::collections::BTreeMap;
use core::ops::AddAssign;
use num::traits::{ Zero, One};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}

#[derive(Debug)]
pub struct Pallet<T : Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>
}

impl<T: Config> Pallet<T>
{

    pub fn new() -> Self {
        Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
    }

    pub fn block_number (&self) -> T::BlockNumber{
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        let new_nonce = nonce + T::Nonce::one();
        self.nonce.insert(who.clone(), new_nonce);
    }

}

#[cfg(test)]
mod test {
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }
	#[test]
	fn init_system() {
		let mut Pallet = super::Pallet::<TestConfig>::new();
		Pallet.inc_block_number();
		Pallet.inc_nonce(&"alice".to_string());

		assert_eq!(Pallet.block_number(), 1);
		assert_eq!(Pallet.nonce.get("alice"), Some(&1));
		assert_eq!(Pallet.nonce.get("bob"), None);
	}
}