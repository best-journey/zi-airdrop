#![cfg(test)]
extern crate std;

use crate::contract::ZiAirdropClient;

#[cfg(test)]

mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_distribute_tokens() {
        let e = Env::default();
        let contract_id = e.register_contract(None, ZiAirdropClient {});
        let airdrop = ZiAirdropClient::new(&e, &contract_id);
    }
}
