mod balances;
mod system;
mod support;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type Nonce = u32;
	pub type BlockNumber = u32;
}


#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl Runtime {
	fn new() -> Self {
		Self {
			system : system::Pallet::new(),
			balances: balances::Pallet::new(),
		}
	}
}


fn main() {
	println!("Hello, world!");

	//Create a mutable variable `runtime`, which is a new instance of `Runtime`.
	let mut runtime = Runtime::new();


	//Set the balance of `alice` to 100, allowing us to execute other transactions.
	runtime.balances.set_balance(&"alice".to_string(), 100);

	//Increment the block number in system.
	runtime.system.inc_block_number();

	//Assert the block number is what we expect.
	assert_eq!(runtime.system.block_number(),1);

	//Increment the nonce of `alice`. */
	runtime.system.inc_nonce(&"alice".to_string());

	// TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
	runtime.balances.set_balance(&"bob".to_string(), 0);
	let _res = runtime
		.balances
		.transfer("alice".to_string(),"bob".to_string(),30)
		.map_err(|e| eprintln!("{}", e));
	
	// second transaction
    /* TODO: Increment the nonce of `alice` again. */
	runtime.system.inc_nonce(&"alice".to_string());
    /* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
	let _res_two = runtime
		.balances
		.transfer("alice".to_string(),"charlie".to_string(),20)
		.map_err(|e| eprintln!("{}", e));

	/* TODO: Print the final runtime state after all transactions. */
	println!("{:#?}", runtime);

}


#[cfg(test)]
mod tests {
	use super::balances;
    use super::system;
	//initialize a new instance of our Pallet
	struct TestConfig;
	struct BalanceTestConfig;
	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}
	impl super::Config for BalanceTestConfig {
		type Balance = u128;
	}
	#[test]
	fn init_balances() { 
		let mut balances = balances::Pallet::<BalanceTestConfig>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		//TODO: Create a test that checks the following:
		let mut balances = balances::Pallet::<BalanceTestConfig>::new();

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

	#[test]
	fn init_system() {
	
		//TODO: Create a test which checks the following:

		let mut mutual_pallet = system::Pallet::<TestConfig>::new();

		//Increment the current block number.
		mutual_pallet.inc_block_number();

		//Increment the nonce of `alice`.
		mutual_pallet.inc_nonce(&"alice".to_string());

		//Check the block number is what we expect.
		assert_eq!(mutual_pallet.block_number(),1);

		//Check the nonce of `alice` is what we expect.
		assert_eq!(mutual_pallet.get_nonce(&"alice".to_string()),Some(&1));

		//Check the nonce of `bob` is what we expect.
		assert_eq!(mutual_pallet.get_nonce(&"bob".to_string()), None);
	}
}
