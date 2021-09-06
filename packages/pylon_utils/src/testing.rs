use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{Coin, Decimal, Uint128};
use terra_cosmwasm::TerraQuerier;

use crate::mock_querier::mock_dependencies;
use crate::mock_tax::MockTax;
use crate::tax::{compute_tax, deduct_tax};

#[test]
fn tax_rate_querier() {
    let mut deps = mock_dependencies(20, &[]);

    deps.querier
        .with_tax(MockTax::new(Decimal::percent(1), &[]));

    assert_eq!(
        Decimal256::from(
            TerraQuerier::new(&deps.querier)
                .query_tax_rate()
                .unwrap()
                .rate
        ),
        Decimal256::percent(1),
    );
}

#[test]
fn test_compute_tax() {
    let mut deps = mock_dependencies(20, &[]);

    deps.querier.with_tax(MockTax::new(
        Decimal::percent(1),
        &[(&"uusd".to_string(), &Uint128::from(1000000u128))],
    ));

    // cap to 1000000
    assert_eq!(
        compute_tax(&deps, &Coin::new(10000000000u128, "uusd")).unwrap(),
        Uint256::from(1000000u64)
    );

    // normal tax
    assert_eq!(
        compute_tax(&deps, &Coin::new(50000000u128, "uusd")).unwrap(),
        Uint256::from(495049u64)
    );
}

#[test]
fn test_deduct_tax() {
    let mut deps = mock_dependencies(20, &[]);

    deps.querier.with_tax(MockTax::new(
        Decimal::percent(1),
        &[(&"uusd".to_string(), &Uint128::from(1000000u128))],
    ));

    // cap to 1000000
    assert_eq!(
        deduct_tax(&deps, Coin::new(10000000000u128, "uusd")).unwrap(),
        Coin {
            denom: "uusd".to_string(),
            amount: Uint128(9999000000u128)
        }
    );

    // normal tax
    assert_eq!(
        deduct_tax(&deps, Coin::new(50000000u128, "uusd")).unwrap(),
        Coin {
            denom: "uusd".to_string(),
            amount: Uint128(49504951u128)
        }
    );
}
