use crate::types::*;
use candid::Principal;
use ic_cdk::{post_upgrade, pre_upgrade, storage};
use state::State;
use std::cell::RefCell;
mod admin;
mod asset;
mod asset_certification;
mod canisters;
mod evidence;
mod state;
mod state_machine;
mod store_state;
mod store_types;
mod types;
mod url_decode;
//new branch
thread_local! {
    static STATE: RefCell<State> = RefCell::new(Default::default());
}

#[ic_cdk_macros::init]
fn init() {
    init_hook();
}

fn init_hook() {
    // STATE.with(|state| {
    //     let mut state = state.borrow_mut();
    // });
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        storage::stable_save((State {
            temp_asset_canister: state.borrow().temp_asset_canister,
            provision_canister: state.borrow().provision_canister,
            admins: state.borrow().admins.clone(),
        },))
        .unwrap()
    });
}

#[post_upgrade]
fn post_upgrade() {
    let state: Result<(State,), String> = storage::stable_restore();
    match state {
        Ok(state) => {
            STATE.with(|s| {
                *s.borrow_mut() = state.0;
            });
            init_hook();
        }
        Err(e) => {
            println!("Failed to do post upgrade {e}");
        }
    }
}

ic_cdk_macros::export_candid!();
