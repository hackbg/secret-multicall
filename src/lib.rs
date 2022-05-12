mod interfaces;

use fadroma::derive_contract::{contract_impl, init, query};
use fadroma::{
    cosmwasm_std, to_vec, Api, Empty, Extern, InitResponse, Querier, QuerierResult, QueryRequest,
    QueryResult, StdResult, Storage, SystemResult, WasmQuery,
};
use interfaces::multicall::{ChainResponse, MapResponse, MultiQuery, MultiQueryResult};
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

fn process_wasm_query<S, A, Q>(
    deps: &Extern<S, A, Q>,
    request: MultiQuery,
) -> StdResult<MultiQueryResult>
where
    S: Storage,
    A: Api,
    Q: Querier,
{
    let msg: QueryRequest<Empty> = WasmQuery::Smart {
        contract_addr: request.contract_address,
        callback_code_hash: "".to_string(),
        msg: request.query,
    }
    .into();

    let query_response: QuerierResult = deps.querier.raw_query(&to_vec(&msg)?);

    let result = match query_response {
        Ok(Ok(data)) => MultiQueryResult {
            error: None,
            data: Some(data),
        },
        Ok(Err(err)) => MultiQueryResult {
            error: Some(err.to_string()),
            data: None,
        },
        Err(err) => MultiQueryResult {
            error: Some(err.to_string()),
            data: None,
        },
    };

    Ok(result)
}
