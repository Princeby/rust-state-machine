mod balances;
mod system;
mod support;

use crate::support::Dispatch;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type Nonce = u32;
	pub type BlockNumber = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

//The calls that are exposed to the world.
pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
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

	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		//Increment the system's block number
		self.system.inc_block_number();
		if block.header.block_number != self.system.block_number() {
			return Err("block number does not match what is expected")
		}
		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}
		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	fn dispatch(
		&mut self, 
		caller: Self::Caller, 
		runtime_call: Self::Call
	) -> support::DispatchResult {
		match runtime_call {
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)?;
			},
		}
		Ok(())
	}
	
}


fn main() {

	//Create a mutable variable `runtime`, which is a new instance of `Runtime`.
	let mut runtime = Runtime::new();

	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();


	//Set the balance of `alice` to 100, allowing us to execute other transactions.
	runtime.balances.set_balance(&alice, 100);

	let block_1 = types::Block {
		header: support::Header { block_number: 1},
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: bob, amount: 30 }),
			},
			support::Extrinsic {
				caller: alice,
				call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie, amount: 30 }),
			},
		],
	};

	runtime.execute_block(block_1).expect("invalid block");

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
