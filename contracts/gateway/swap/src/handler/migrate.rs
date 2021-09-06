use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::{Api, Env, Extern, HumanAddr, MigrateResponse, MigrateResult, Querier, Storage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::config::KEY_CONFIG;
use crate::state::{config, vpool};
use cosmwasm_storage::Singleton;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NewRefundConfig {
    pub manager: HumanAddr,
    pub refund_denom: String,
    pub base_price: Decimal256,
}

pub fn refund<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>, _: Env) -> MigrateResult {
    let config = config::read(&deps.storage).unwrap();
    let vpool = vpool::read(&deps.storage).unwrap();

    Singleton::new(&mut deps.storage, KEY_CONFIG)
        .save(&NewRefundConfig {
            manager: config.owner.clone(),
            refund_denom: vpool.x_denom,
            base_price: config.base_price,
        })
        .unwrap();

    Ok(MigrateResponse::default())
}
