use crate::admin::{
    has_administrator, read_administrator, read_config, write_administrator, write_config, Config
};
use soroban_sdk::{contract, contractimpl, token::TokenClient, Address, Env};

#[contract]
pub struct ZiAirdrop;

#[contractimpl]
impl ZiAirdrop {
    pub fn initialize(e: &Env, admin: Address) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
    }

    pub fn set_config(e: &Env, config: Config) {
        let admin = read_administrator(&e);
        admin.require_auth();
        write_config(&e, &config);
    }

    pub fn distribute_tokens(
        e: Env,
        sender: Address,
        recipient: Address,
        action: u32,
    ) -> bool {
        sender.require_auth();

        assert!(
            Self::is_performed_action(&e, recipient.clone(), action) == false,
            "You've already received this airdrop"
        );

        let amount = Self::get_airdrop_amount(&e, action);
        assert!(amount > 0, "This airdrop is not allowed");

        let config = read_config(&e);

        let token = TokenClient::new(&e, &config.zi);
        token.transfer(&sender, &recipient, &amount);

        Self::set_is_performed_action(&e, &recipient, action);
        true
    }

    pub fn get_airdrop_amount(e: &Env, action: u32) -> i128 {
        e.storage().instance().get(&action).unwrap_or(0i128)
    }

    pub fn set_airdrop_amount(e: &Env, action: u32, amount: i128) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage().instance().set(&action, &amount);
    }

    pub fn is_performed_action(e: &Env, user: Address, action: u32) -> bool {
        let key = (user.clone(), action);
        e.storage().instance().has(&key)
    }

    fn set_is_performed_action(e: &Env, user: &Address, action: u32) {
        let key = (user.clone(), action);
        e.storage().instance().set(&key, &true);
    }
}
