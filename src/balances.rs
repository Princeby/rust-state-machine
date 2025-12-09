use std::collections::BTreeMap;
pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
    //Initialize a new user balance
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

    //Set new user balance
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    //Get User balance
    pub fn get_balance(&mut self, who: &String) {
        *self.balances.get(who).unwrap_or(&0);
    }


}
