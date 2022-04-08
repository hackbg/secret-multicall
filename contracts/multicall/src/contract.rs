mod querier;

use crate::querier::{
        aggregrate, block_aggregrate, block_try_aggregate_optional, block_try_aggregrate,
        try_aggregate, try_aggregate_optional,
    };
use cosmwasm_std::{to_binary, Binary};
use fadroma::{auth::admin, cosmwasm_std, derive_contract::*, schemars, InitResponse, StdResult};

#[contract(entry, component(path = "admin"))]
pub trait Multicall {
    #[init]
    fn init() -> StdResult<InitResponse> {
        let mut response = admin::Admin::new(&admin::DefaultImpl, None, deps, env)?;

        Ok(response)
    }

    #[query]
    fn query(msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Aggregate { queries } => to_binary(&aggregrate(deps, queries)?),
            QueryMsg::TryAggregate {
                require_success,
                include_cause,
                queries,
            } => to_binary(&try_aggregate(
                deps,
                require_success,
                include_cause,
                queries,
            )?),
            QueryMsg::TryAggregateOptional {
                include_cause,
                queries,
            } => to_binary(&try_aggregate_optional(deps, include_cause, queries)?),
            QueryMsg::BlockAggregate { queries } => {
                to_binary(&block_aggregrate(deps, env, queries)?)
            }
            QueryMsg::BlockTryAggregate {
                require_success,
                include_cause,
                queries,
            } => to_binary(&block_try_aggregrate(
                deps,
                env,
                require_success,
                include_cause,
                queries,
            )?),
            QueryMsg::BlockTryAggregateOptional {
                include_cause,
                queries,
            } => to_binary(&block_try_aggregate_optional(
                deps,
                env,
                include_cause,
                queries,
            )?),
        }
    }
}
