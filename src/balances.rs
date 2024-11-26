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

    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
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