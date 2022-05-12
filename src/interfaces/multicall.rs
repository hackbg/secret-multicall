use fadroma::{
    cosmwasm_std,
    derive_contract::{init, interface, query},
    Binary, HumanAddr, InitResponse, StdResult,
};
use schemars::{JsonSchema, Map};
use serde::{Deserialize, Serialize};

#[interface]
pub trait Multicall {
    #[query]
    fn version() -> StdResult<String>;
    #[init]
    fn new() -> StdResult<InitResponse>;
    #[query]
    fn multi_chain(queries: Vec<MultiQuery>) -> StdResult<ChainResponse>;
    #[query]
    fn multi_map(queries: Map<String, MultiQuery>) -> StdResult<MapResponse>;
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct MultiQuery {
    pub contract_address: HumanAddr,
    pub query: Binary,
}
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct QueryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<Binary>,
}

pub type MapResponse = Map<String, QueryResult>;
pub type ChainResponse = Vec<QueryResult>;
