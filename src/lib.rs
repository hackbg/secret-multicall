mod interfaces;

use fadroma::derive_contract::{contract_impl, init, query};
use fadroma::{cosmwasm_std, InitResponse, StdResult};
use interfaces::multicall::{ChainResponse, MapResponse, MultiQuery};
use schemars::Map;

// MAKE SURE TO CHANGE ON ANY NEW DEPLOY
const VERSION: &str = "0.0.1";

#[contract_impl(entry, path = "interfaces::multicall")]
pub trait Multicall {
    #[init]
    fn new() -> StdResult<InitResponse> {
        Ok(InitResponse::default())
    }

    #[query]
    fn version() -> StdResult<String> {
        Ok(VERSION.to_string())
    }

    #[query]
    fn multi_chain(queries: Vec<MultiQuery>) -> StdResult<ChainResponse> {
        Ok(ChainResponse::default())
    }

    #[query]
    fn multi_map(queries: Map<String, MultiQuery>) -> StdResult<MapResponse> {
        Ok(MapResponse::default())
    }
}
