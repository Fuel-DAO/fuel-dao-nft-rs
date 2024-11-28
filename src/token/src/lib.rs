mod state;
mod validations;
mod permissions;
mod ports;
use std::cell::RefCell;
use candid::Principal;
use ic_cdk::storage;
use state::MetaDataState;
use state::State;
use state::metadata::Metadata;
use ic_cdk_macros::*;
use state::models::*;
use crate::state::metadata::*;
use crate::state::escrow::SaleStatus;
use candid::Nat;
use crate::state::icrc7::ICRC7MetadataQueryResult;
use crate::state::supported_standards::SupportedStandard;


thread_local! {
    static STATE: RefCell<State> = RefCell::new(Default::default());
}


#[ic_cdk_macros::init]
fn init(base: Metadata) {
    init_hook(base);
}

fn init_hook(meta: Metadata) {
    STATE.with_borrow_mut(| state| {
     *state = State {
        metadata: Some(MetaDataState {
            metadata: meta,
            total_supply: 0, 
        }),
        ..Default::default()
 };
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| storage::stable_save((State {
        metadata: state.borrow().metadata.clone(), 
        escrow: state.borrow().escrow.clone(), 
        transactions: state.borrow().transactions.clone(),
        tokens: state.borrow().tokens.clone(),
    },)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let state: Result<(State, ), String> = storage::stable_restore();
    match state {
        Ok(state) => {
            STATE.with(|s| { *s.borrow_mut() =  state.0;  });
        }, Err(e) => {
            println!("Failed to do post upgrade {e}");
        }
    }
}

ic_cdk_macros::export_candid!();
