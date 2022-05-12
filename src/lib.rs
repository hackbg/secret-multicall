use fadroma::derive_contract::{contract, init, query};
use fadroma::{cosmwasm_std, InitResponse, StdResult};

// MAKE SURE TO CHANGE ON ANY NEW DEPLOY
const VERSION: &str = "0.0.1";

#[contract(entry)]
pub trait Multicall {
    #[init]
    fn new() -> StdResult<InitResponse> {
        Ok(InitResponse::default())
    }

    #[query]
    fn version() -> StdResult<String> {
        Ok(VERSION.to_string())
    }
}
