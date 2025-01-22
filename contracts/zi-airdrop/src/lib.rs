use soroban_sdk::{contract, contractimpl, Address, ConversionError, Env, TryFromVal, Val};

#[derive(Clone, Copy, Debug)]
pub enum Action {
    SpinCube,
    CreateParticles,
    ChangeTheme,
}

#[contract]
pub struct ZiAirdrop;

#[contractimpl]
impl ZiAirdrop {
    pub fn distribute_tokens(env: &Env, user: Address, action: Action) {
        // Store the updated balance in storage
        let reward = match action {
            Action::SpinCube => 2,
            Action::CreateParticles => 2,
            Action::ChangeTheme => 1,
        };

        // Retrieve the current balance of the user from storage
        let balance = Self::get_balance(&env, user.clone());

        // Update the user's balance
        if balance + reward < 5 {
            Self::set_balance(&env, user, balance + reward)
        } else {
            Self::set_balance(&env, user, 5);
        }
    }

    pub fn get_balance(env: &Env, user: Address) -> u32 {
        // Try to get the balance from storage, default to 0 if not set
        let storage = env.storage().instance();
        match storage.get(&user) {
            Some(balance) => balance,
            None => 0,
        }
    }

    pub fn set_balance(env: &Env, user: Address, amount: u32) {
        // Store the updated balance in storage
        let storage = env.storage().instance();
        storage.set(&user, &amount);
    }
}

impl TryFromVal<Env, Action> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &Action) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

impl TryFromVal<Env, Val> for Action {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &Val) -> Result<Self, Self::Error> {
        Ok(Action::SpinCube)
    }
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(ZiAirdrop, ());
    let client = ZiAirdropClient::new(&env, &contract_id);

    let words = client.distribute_tokens(&symbol_short!("Dev"));

    assert_eq!(words, vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]);
}