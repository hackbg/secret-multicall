use fadroma::{
    cosmwasm_std,
    derive_contract::{init, interface, query},
    prelude::*,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[interface]
pub trait Multicall {
    #[query]
    fn version() -> StdResult<String>;
    #[init]
    fn new() -> StdResult<InitResponse>;
    #[query]
    fn multi_chain(queries: Vec<MultiQuery>) -> StdResult<ChainResponse>;
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MultiQuery {
    pub contract_address: HumanAddr,
    pub code_hash: String,
    pub query: Binary,
}
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MultiQueryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Binary>,
}

pub type ChainResponse = Vec<MultiQueryResult>;
