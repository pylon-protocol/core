use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{
    to_binary, Api, CanonicalAddr, CosmosMsg, Extern, Querier, QueryRequest, StdResult, Storage,
    WasmQuery,
};

use pylon_core::adapter::{ConfigResponse, ExchangeRateResponse, QueryMsg as AdapterQueryMsg};

pub fn config<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    adapter: &CanonicalAddr,
) -> StdResult<ConfigResponse> {
    deps.querier
        .query::<ConfigResponse>(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.human_address(adapter)?,
            msg: to_binary(&AdapterQueryMsg::Config {})?,
        }))
}

pub fn exchange_rate<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    adapter: &CanonicalAddr,
    input_denom: &String,
) -> StdResult<Decimal256> {
    let resp = deps
        .querier
        .query::<ExchangeRateResponse>(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.human_address(adapter)?,
            msg: to_binary(&AdapterQueryMsg::ExchangeRate {
                input_denom: input_denom.clone(),
            })?,
        }))?;
    Ok(resp.exchange_rate)
}

pub fn deposit<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    adapter: &CanonicalAddr,
    amount: Uint256,
) -> StdResult<Vec<CosmosMsg>> {
    deps.querier
        .query::<Vec<CosmosMsg>>(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.human_address(adapter)?,
            msg: to_binary(&AdapterQueryMsg::Deposit { amount })?,
        }))
}

pub fn redeem<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    adapter: &CanonicalAddr,
    amount: Uint256,
) -> StdResult<Vec<CosmosMsg>> {
    deps.querier
        .query::<Vec<CosmosMsg>>(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.human_address(adapter)?,
            msg: to_binary(&AdapterQueryMsg::Redeem { amount })?,
        }))
}