use crate::config::{read_config, store_config, Config};
use crate::lib_anchor::{market_deposit_stable_msg, market_epoch_state, market_redeem_stable_msg};
use crate::lib_pool::{calculate_return_amount, calculate_reward_amount};
use crate::msg::Cw20HookMsg;
use cosmwasm_bignumber::Uint256;
use cosmwasm_std::{
    from_binary, log, to_binary, Api, BankMsg, CanonicalAddr, Coin, CosmosMsg, Env, Extern,
    HandleResponse, HumanAddr, Querier, StdError, StdResult, Storage, Uint128, WasmMsg,
};
use cw20::{Cw20HandleMsg, Cw20ReceiveMsg};
use moneymarket::querier::deduct_tax;

pub fn handle_receive<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    _env: Env,
    cw20_msg: Cw20ReceiveMsg,
) -> StdResult<HandleResponse> {
    let sender = _env.message.sender.clone();
    if let Some(msg) = cw20_msg.msg {
        match from_binary(&msg)? {
            Cw20HookMsg::Redeem {} => {
                // only asset contract can execute this message
                let config: Config = read_config(&deps.storage)?;
                if deps.api.canonical_address(&sender)? != config.dp_token {
                    return Err(StdError::unauthorized());
                }

                handle_redeem(deps, _env, cw20_msg.sender, cw20_msg.amount)
            }
        }
    } else {
        Err(StdError::generic_err(
            "Invalid request: \"redeem\" message not included in request",
        ))
    }
}

pub fn handle_deposit<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    _env: Env,
) -> StdResult<HandleResponse> {
    let config: Config = read_config(&deps.storage)?;

    // check deposit
    let deposit_amount: Uint256 = _env
        .message
        .sent_funds
        .iter()
        .find(|c| c.denom == config.stable_denom)
        .map(|c| Uint256::from(c.amount))
        .unwrap_or_else(Uint256::zero);

    if deposit_amount.is_zero() {
        return Err(StdError::generic_err(format!(
            "Pool: insufficient token amount {}",
            config.stable_denom,
        )));
    }

    Ok(HandleResponse {
        messages: [
            market_deposit_stable_msg(
                deps,
                &config.moneymarket,
                &config.stable_denom,
                deposit_amount.into(),
            )?,
            vec![CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: deps.api.human_address(&config.dp_token)?,
                msg: to_binary(&Cw20HandleMsg::Mint {
                    recipient: _env.message.sender.clone(),
                    amount: deposit_amount.into(),
                })?,
                send: vec![],
            })],
        ]
        .concat(),
        log: vec![
            log("action", "deposit"),
            log("operator", _env.message.sender.clone()),
            log("amount", deposit_amount.to_string()),
        ],
        data: None,
    })
}

pub fn handle_redeem<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    _env: Env,
    sender: HumanAddr,
    amount: Uint128,
) -> StdResult<HandleResponse> {
    let config: Config = read_config(&deps.storage)?;

    let epoch_state = market_epoch_state(deps, &config.moneymarket)?;

    let market_redeem_amount = Uint256::from(amount) / epoch_state.exchange_rate; // calculate
    let pool_redeem_amount = deduct_tax(
        deps,
        Coin {
            denom: config.stable_denom.clone(),
            amount: market_redeem_amount.into(),
        },
    )?;
    let return_amount = deduct_tax(deps, pool_redeem_amount)?;

    Ok(HandleResponse {
        messages: [
            vec![CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: deps.api.human_address(&config.dp_token)?,
                msg: to_binary(&Cw20HandleMsg::Burn { amount })?,
                send: vec![],
            })],
            market_redeem_stable_msg(
                deps,
                &config.moneymarket,
                &config.atoken,
                market_redeem_amount.into(),
            )?,
            vec![CosmosMsg::Bank(BankMsg::Send {
                from_address: _env.contract.address,
                to_address: sender,
                amount: vec![return_amount.clone()],
            })],
        ]
        .concat(),
        log: vec![
            log("action", "redeem"),
            log("operator", _env.message.sender.clone()),
            log("amount", return_amount.amount.clone()),
        ],
        data: None,
    })
}

pub fn handle_claim_reward<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    _env: Env,
) -> StdResult<HandleResponse> {
    // calculate (total_aust_amount * exchange_rate) - (total_dp_balance)
    let config: Config = read_config(&deps.storage)?;
    if config.beneficiary != deps.api.canonical_address(&_env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    let reward_amount = calculate_reward_amount(deps)?;
    let (market_redeem_amount, _, return_amount) =
        calculate_return_amount(deps, reward_amount.clone())?;

    Ok(HandleResponse {
        messages: [
            market_redeem_stable_msg(
                deps,
                &config.moneymarket,
                &config.atoken,
                market_redeem_amount.into(),
            )?,
            vec![CosmosMsg::Bank(BankMsg::Send {
                from_address: _env.contract.address.clone(),
                to_address: _env.message.sender.clone(),
                amount: vec![return_amount.clone()],
            })],
        ]
        .concat(),
        log: vec![
            log("action", "claim_reward"),
            log("operator", _env.message.sender.clone()),
            log("amount", return_amount.clone().amount),
        ],
        data: None,
    })
}

pub fn handle_register_dp_token<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> StdResult<HandleResponse> {
    let mut config: Config = read_config(&deps.storage)?;
    if config.dp_token != CanonicalAddr::default() {
        return Err(StdError::unauthorized());
    }

    config.dp_token = deps.api.canonical_address(&_env.message.sender)?;
    store_config(&mut deps.storage, &config)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("dp_token", _env.message.sender)],
        data: None,
    })
}