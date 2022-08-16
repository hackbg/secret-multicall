mod interfaces;
#[cfg(test)]
mod tests;

use fadroma::derive_contract::{contract_impl, init, query};
use fadroma::{cosmwasm_std, prelude::*};
use interfaces::multicall::{ChainResponse, MultiQuery, MultiQueryResult};

// MAKE SURE TO CHANGE ON ANY NEW DEPLOY
const VERSION: &str = "0.0.1";

#[contract_impl(entry, path = "crate::interfaces::multicall")]
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
    fn batch_query(queries: Vec<MultiQuery>) -> StdResult<ChainResponse> {
        let results = queries
            .into_iter()
            .map(|query| process_wasm_query(deps, query))
            .collect::<Vec<_>>();

        Ok(results)
    }
}

fn process_wasm_query<S, A, Q>(deps: &Extern<S, A, Q>, request: MultiQuery) -> MultiQueryResult
where
    S: Storage,
    A: Api,
    Q: Querier,
{
    let msg: QueryRequest<Empty> = WasmQuery::Smart {
        contract_addr: request.contract_address,
        callback_code_hash: request.code_hash.clone(),
        msg: request.query,
    }
    .into();

    let query_response: QuerierResult = deps.querier.raw_query(&to_vec(&msg).unwrap());

    match query_response {
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
    }
}
