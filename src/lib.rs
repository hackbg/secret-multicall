mod interfaces;
mod state;
#[cfg(test)]
mod tests;

use fadroma::derive_contract::{contract_impl, init, query};
use fadroma::{
    cosmwasm_std, to_vec, Api, ContractLink, Empty, Extern, HumanAddr, InitResponse, Querier,
    QuerierResult, QueryRequest, StdResult, Storage, WasmQuery,
};
use interfaces::multicall::{ChainResponse, MultiQuery, MultiQueryResult};
use state::{load_self_ref, save_self_ref};

// MAKE SURE TO CHANGE ON ANY NEW DEPLOY
const VERSION: &str = "0.0.1";

#[contract_impl(entry, path = "crate::interfaces::multicall")]
pub trait Multicall {
    #[init]
    fn new() -> StdResult<InitResponse> {
        save_self_ref(
            deps,
            ContractLink {
                address: env.contract.address,
                code_hash: env.contract_code_hash,
            },
        )?;
        Ok(InitResponse::default())
    }

    #[query]
    fn version() -> StdResult<String> {
        Ok(VERSION.to_string())
    }

    #[query]
    fn multi_chain(queries: Vec<MultiQuery>) -> StdResult<ChainResponse> {
        let self_ref = load_self_ref(deps)?;
        let results = queries
            .into_iter()
            .map(|query| process_wasm_query(deps, query, &self_ref))
            .collect::<Vec<_>>();

        Ok(results)
    }
}

fn process_wasm_query<S, A, Q>(
    deps: &Extern<S, A, Q>,
    request: MultiQuery,
    self_ref: &ContractLink<HumanAddr>,
) -> MultiQueryResult
where
    S: Storage,
    A: Api,
    Q: Querier,
{
    let msg: QueryRequest<Empty> = WasmQuery::Smart {
        contract_addr: request.contract_address,
        callback_code_hash: self_ref.code_hash.clone(),
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
