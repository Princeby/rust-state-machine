mod balances;
mod system;


mod types {
    pub type AccountId = String;
    pub type Balance = u128;
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet<types::AccountId, types::Balance>,
}


impl Runtime {
    fn new() -> Self { 
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new()
        }
    }
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	runtime.balances.set_balance(&alice, 100);

	// start emulating a block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	runtime.system.inc_nonce(&alice);
	let _res = runtime.balances.transfer(alice.clone(), bob, 30).map_err(|e| eprintln!("{e}"));

	// second transaction
	runtime.system.inc_nonce(&alice);
	let _res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| eprintln!("{e}"));

	/* TODO: Print the final runtime state after all transactions. */

    println!("{:#?}", runtime);
}