#![no_std]
use soroban_sdk::{contract, contractimpl, Address, ConversionError, Env, TryFromVal, Val};

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Unknown = 0,
    SpinCube = 1,
    CreateParticles = 2,
    ChangeTheme = 3,
}

impl TryFromVal<Env, Action> for Val {
    type Error = ConversionError;

    fn try_from_val(_e: &Env, v: &Action) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

impl TryFromVal<Env, Val> for Action {
    type Error = ConversionError;

    fn try_from_val(e: &Env, v: &Val) -> Result<Self, Self::Error> {
        let val: u32 = u32::try_from_val(e, v)?;

        match val {
            1 => Ok(Action::SpinCube),
            2 => Ok(Action::CreateParticles),
            3 => Ok(Action::ChangeTheme),
            _ => Ok(Action::Unknown),
        }
    }
}

#[contract]
pub struct ZiAirdrop;

#[contractimpl]
impl ZiAirdrop {
    pub fn distribute_tokens(e: &Env, user: Address, action: Action) {
        // Check if the action is valid and has not been performed before
        if Self::has_performed_action(e, &user, action) {
            panic!("You have already received this type of airdrop.");
        }

        let reward = match action {
            Action::Unknown => 0,
            Action::SpinCube => 2,
            Action::CreateParticles => 2,
            Action::ChangeTheme => 1,
        };

        // Retrieve the current balance of the user from storage
        let balance = Self::get_balance(e, user.clone());

        // Update the user's balance
        let new_balance = balance + reward;
        Self::set_balance(e, user.clone(), new_balance);

        // Mark the action as performed
        Self::mark_action_performed(e, &user, action);
    }

    pub fn get_balance(e: &Env, addr: Address) -> u128 {
        // Try to get the balance from storage, default to 0 if not set
        if let Some(balance) = e.storage().persistent().get(&addr) {
            balance
        } else {
            0
        }
    }

    pub fn set_balance(e: &Env, addr: Address, amount: u128) {
        // Store the updated balance in storage
        e.storage().persistent().set(&addr, &amount);
    }

    fn has_performed_action(e: &Env, user: &Address, action: Action) -> bool {
        let key = (user.clone(), action);
        e.storage().instance().has(&key)
    }

    fn mark_action_performed(e: &Env, user: &Address, action: Action) {
        let key = (user.clone(), action);
        e.storage().instance().set(&key, &true);
    }
}

#[cfg(test)]

mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_distribute_tokens() {
        let e = Env::default();
        let contract_id = e.register_contract(None, ZiAirdrop);
        let client = ZiAirdropClient::new(&e, &contract_id);

        let user = Address::from_str(&e, "GAYW7P7VQ6BDSSW4TJOIXX5CITVFG455NQIAMCK22BYUOUHI6CKOAEFV");

        // Distribute tokens for action 1 (SpinCube)
        client.distribute_tokens(&user, &Action::SpinCube);
        assert_eq!(client.get_balance(&user), 2);

        // Try to distribute tokens for action 1 again (should not change balance)
        client.distribute_tokens(&user, &Action::SpinCube);
        assert_eq!(client.get_balance(&user), 2);

        // Distribute tokens for action 2 (CreateParticles)
        client.distribute_tokens(&user, &Action::CreateParticles);
        assert_eq!(client.get_balance(&user), 4);

        // Distribute tokens for action 3 (ChangeTheme)
        client.distribute_tokens(&user, &Action::ChangeTheme);
        assert_eq!(client.get_balance(&user), 5);

        // Try to distribute tokens for action 2 again (should not change balance)
        client.distribute_tokens(&user, &Action::CreateParticles);
        assert_eq!(client.get_balance(&user), 5);
    }
}
