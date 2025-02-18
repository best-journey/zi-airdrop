#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, token::TokenClient, Address, ConversionError, Env, TryFromVal, Val};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyReceived = 1,
}

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
    pub fn distribute_tokens(e: Env, token_id: Address, sender: Address, recipient: Address, action: Action) -> Result<bool, Error> {
        if Self::has_performed_action(&e, &recipient, action) {
            return Err(Error::AlreadyReceived);
        }

        let reward = match action {
            Action::Unknown => 0,
            Action::SpinCube => 20000000,
            Action::CreateParticles => 20000000,
            Action::ChangeTheme => 10000000,
        };

        sender.require_auth();

        let token = TokenClient::new(&e, &token_id);
        token.transfer(&sender, &recipient, &reward);

        Self::mark_action_performed(&e, &recipient, action);
        Ok(true)
    }

    fn has_performed_action(e: &Env, user: &Address, action: Action) -> bool {
        let key = (user.clone(), action);
        e.storage().instance().has(&key)
    }

    pub fn get_status(e: Env, user: Address) -> u128 {
        if Self::has_performed_action(&e, &user, Action::ChangeTheme) {
            return 3;
        }

        if Self::has_performed_action(&e, &user, Action::CreateParticles) {
            return 2;
        }

        if Self::has_performed_action(&e, &user, Action::SpinCube) {
            return 1;
        }

        0
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
    }
}
