use fadroma::{
    derive_contract::interface, Binary, HandleResponse, HumanAddr, InitResponse, StdResult,
};
use schemars::{JsonSchema, Map};
use serde::{Deserialize, Serialize};

#[interface]
pub trait Multicall {
    fn version() -> String;
    fn new() -> StdResult<InitResponse>;
    fn multi_chain(queries: Vec<MultiQuery>) -> StdResult<HandleResponse>;
    fn multi_map() -> StdResult<HandleResponse>;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MultiQuery {
    pub contract_address: HumanAddr,
    pub query: Binary,
}
#[derive(Serialize, Deserialize, JsonSchema)]
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
