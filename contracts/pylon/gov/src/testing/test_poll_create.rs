use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{to_binary, StdError, Uint128};
use cw20::Cw20ReceiveMsg;
use pylon_token::gov_msg::{Cw20HookMsg, ExecuteMsg};

use crate::contract;
use crate::error::ContractError;
use crate::testing::assert::assert_create_poll_result;
use crate::testing::constants::*;
use crate::testing::message::create_poll_msg;
use crate::testing::mock_querier::mock_dependencies;
use crate::testing::utils::{mock_env_height, mock_instantiate};

#[test]
fn create_poll_no_quorum() {
    let mut deps = mock_dependencies(&[]);
    mock_instantiate(deps.as_mut());

    let info = mock_info(VOTING_TOKEN, &[]);
    let env = mock_env_height(0, 10000);

    let msg = create_poll_msg(None, None, None, None, None);
    let execute_res = contract::execute(deps.as_mut(), env, info, msg).unwrap();
    assert_create_poll_result(
        1,
        DEFAULT_VOTING_PERIOD,
        TEST_CREATOR,
        execute_res,
        deps.as_ref(),
    );
}

#[test]
fn fails_create_poll_invalid_title() {
    let mut deps = mock_dependencies(&[]);
    mock_instantiate(deps.as_mut());

    let msg = create_poll_msg(Some(SHORT_STRING.to_string()), None, None, None, None);
    let info = mock_info(VOTING_TOKEN, &[]);
    match contract::execute(deps.as_mut(), mock_env(), info.clone(), msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => {
            assert_eq!(msg, "Title too short")
        }
        Err(_) => panic!("Unknown error"),
    }

    let msg = create_poll_msg(Some(LONG_STRING.to_string()), None, None, None, None);
    match contract::execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => {
            assert_eq!(msg, "Title too long")
        }
        Err(_) => panic!("Unknown error"),
    }
}

#[test]
fn fails_create_poll_invalid_category() {
    let mut deps = mock_dependencies(&[]);
    mock_instantiate(deps.as_mut());

    let msg = create_poll_msg(None, Some(SHORT_STRING.to_string()), None, None, None);
    let info = mock_info(VOTING_TOKEN, &[]);
    match contract::execute(deps.as_mut(), mock_env(), info.clone(), msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => {
            assert_eq!(msg, "Category too short")
        }
        Err(_) => panic!("Unknown error"),
    }

    let msg = create_poll_msg(None, Some(LONG_STRING.to_string()), None, None, None);
    match contract::execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => {
            assert_eq!(msg, "Category too long")
        }
        Err(_) => panic!("Unknown error"),
    }
}

#[test]
fn fails_create_poll_invalid_description() {
    let mut deps = mock_dependencies(&[]);
    mock_instantiate(deps.as_mut());

    let msg = create_poll_msg(None, None, Some(SHORT_STRING.to_string()), None, None);
    let info = mock_info(VOTING_TOKEN, &[]);
    match contract::execute(deps.as_mut(), mock_env(), info.clone(), msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => {
            assert_eq!(msg, "Description too short")
        }
        Err(_) => panic!("Unknown error"),
    }

    let msg = create_poll_msg(None, None, Some(LONG_STRING.to_string()), None, None);
    match contract::execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => {
            assert_eq!(msg, "Description too long")
        }
        Err(_) => panic!("Unknown error"),
    }
}

#[test]
fn fails_create_poll_invalid_link() {
    let mut deps = mock_dependencies(&[]);
    mock_instantiate(deps.as_mut());

    let msg = create_poll_msg(None, None, None, Some("http://hih".to_string()), None);
    let info = mock_info(VOTING_TOKEN, &[]);
    match contract::execute(deps.as_mut(), mock_env(), info.clone(), msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => {
            assert_eq!(msg, "Link too short")
        }
        Err(_) => panic!("Unknown error"),
    }

    let msg = create_poll_msg(None, None, None, Some(LONG_STRING.to_string()), None);
    match contract::execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => {
            assert_eq!(msg, "Link too long")
        }
        Err(_) => panic!("Unknown error"),
    }
}

#[test]
fn fails_create_poll_invalid_deposit() {
    let mut deps = mock_dependencies(&[]);
    mock_instantiate(deps.as_mut());

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: TEST_CREATOR.to_string(),
        amount: Uint128::from(DEFAULT_PROPOSAL_DEPOSIT - 1),
        msg: to_binary(&Cw20HookMsg::CreatePoll {
            title: "test".to_string(),
            category: "test".to_string(),
            description: "test".to_string(),
            link: None,
            execute_msgs: None,
        })
        .unwrap(),
    });

    let info = mock_info(VOTING_TOKEN, &[]);
    match contract::execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::InsufficientProposalDeposit(DEFAULT_PROPOSAL_DEPOSIT)) => (),
        Err(_) => panic!("Unknown error"),
    }
}
