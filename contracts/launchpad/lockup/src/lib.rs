// core
pub mod contract;
pub mod msg;
pub mod resp;
pub mod state;

mod handler;
mod lib_staking;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points_with_migration!(contract);