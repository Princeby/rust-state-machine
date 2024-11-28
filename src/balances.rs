use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};


pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances : BTreeMap<T::AccountId, T::Balance>
}

impl <T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances : BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

}

#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> crate::support::DispatchResult {
        //Get the balance of the account caller   
        let caller_balance = self.balance(&caller);
        let receiver_balance = self.balance(&to);

        //Use safe math to calculate a new caller balance
        let new_caller_balance = caller_balance
        .checked_sub(&amount)
        .ok_or("Not enough funds.")?;

        let new_receiver_balance = receiver_balance
        .checked_add(&amount)
        .ok_or("Overflow occured")?;

        //Insert the new balance
        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_receiver_balance);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
	//initialize a new instance of our Pallet
	struct TestConfig;

	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}
	impl super::Config for TestConfig {
		type Balance = u128;
	}
	#[test]
	fn init_balances() { 
		let mut balances = super::Pallet::<TestConfig>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		//TODO: Create a test that checks the following:
		let mut balances = super::Pallet::<TestConfig>::new();

		balances.set_balance(&"alice".to_string(), 0);
		balances.set_balance(&"bob".to_string(), 0);
        //That `alice` cannot transfer funds she does not have.
		assert_eq!(balances.transfer("alice".to_string(),"bob".to_string(),51),
		Err("Not enough funds."));
		balances.set_balance(&"alice".to_string(), 100);

        //That `alice` can successfully transfer funds to `bob`.
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 51),Ok(()));

    	//That the balance of `alice` and `bob` is correctly updated.
		assert_eq!(balances.balance(&"alice".to_string()), 49);
		assert_eq!(balances.balance(&"bob".to_string()), 51);

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);
	}

}
