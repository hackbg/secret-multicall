#![allow(dead_code)]

use fadroma::{
    ensemble::{ContractEnsemble, ContractHarness, MockDeps, MockEnv},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::interfaces::multicall::{ChainResponse, InitMsg, MultiQuery, QueryMsg};
pub struct MulticallTestbed {
    pub ensemble: ContractEnsemble,
    pub mock_contract: ContractLink<HumanAddr>,
    pub multicall: ContractLink<HumanAddr>,
}

impl MulticallTestbed {
    pub fn new() -> Self {
        let mut ensemble = ContractEnsemble::new(200);

        let multicall_model = ensemble.register(Box::new(MulticallContract));
        let mock_model = ensemble.register(Box::new(MockContract));

        let multicall = ensemble
            .instantiate(
                multicall_model.id,
                &InitMsg {},
                MockEnv::new(
                    "ADMIN",
                    ContractLink {
                        address: "Multicall".into(),
                        code_hash: multicall_model.code_hash,
                    },
                ),
            )
            .unwrap();

        let mock_contract = ensemble
            .instantiate(
                mock_model.id,
                &InitMsg {},
                MockEnv::new(
                    "ADMIN",
                    ContractLink {
                        address: "Mock".into(),
                        code_hash: mock_model.code_hash,
                    },
                ),
            )
            .unwrap();

        Self {
            ensemble,
            mock_contract: mock_contract.instance,
            multicall: multicall.instance,
        }
    }
    pub fn batch_query(&self, queries: Vec<QueryMock>) -> ChainResponse {
        let queries = queries
            .into_iter()
            .map(|query| MultiQuery {
                contract_address: self.mock_contract.address.clone(),
                code_hash: self.mock_contract.code_hash.clone(),
                query: to_binary(&query).unwrap(),
            })
            .collect::<Vec<_>>();
        let result: ChainResponse = self
            .ensemble
            .query(
                self.multicall.address.clone(),
                &QueryMsg::MultiChain { queries },
            )
            .unwrap();

        result
    }
}
struct MulticallContract;
impl ContractHarness for MulticallContract {
    fn init(&self, deps: &mut MockDeps, env: Env, msg: Binary) -> StdResult<InitResponse> {
        crate::init(deps, env, from_binary(&msg)?, crate::DefaultImpl)
    }
    fn handle(&self, deps: &mut MockDeps, env: Env, msg: Binary) -> StdResult<HandleResponse> {
        crate::handle(deps, env, from_binary(&msg)?, crate::DefaultImpl)
    }
    fn query(&self, deps: &MockDeps, msg: Binary) -> StdResult<Binary> {
        crate::query(deps, from_binary(&msg)?, crate::DefaultImpl)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum QueryMock {
    Regular { prop: String },
    Exception {},
}

struct MockContract;
impl ContractHarness for MockContract {
    fn init(&self, _deps: &mut MockDeps, _env: Env, _msg: Binary) -> StdResult<InitResponse> {
        Ok(InitResponse::default())
    }
    fn handle(&self, _deps: &mut MockDeps, _env: Env, _msg: Binary) -> StdResult<HandleResponse> {
        Ok(HandleResponse::default())
    }
    fn query(&self, _deps: &MockDeps, msg: Binary) -> StdResult<Binary> {
        let msg = from_binary::<QueryMock>(&msg)?;
        match msg {
            QueryMock::Regular { prop } => Ok(to_binary(&prop)?),
            _ => Err(StdError::generic_err("Exception from query")),
        }
    }
}
