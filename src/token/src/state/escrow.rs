use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::api::call::call;
use serde::Serialize;
use std::collections::HashMap;

use crate::Icrc1Account;

/// Sale Status Enum
#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SaleStatus {
    Live,
    Accepted,
    Rejected,
}

impl Default for SaleStatus {
    fn default() -> Self {
        SaleStatus::Live
    }
}

/// Escrow Store Struct
#[derive(CandidType, Serialize, Deserialize, Debug, Default, Clone)]
pub struct EscrowStore {
    pub sale_status: SaleStatus,
    booked_tokens: HashMap<Principal, u128>, // Using `String` for Principal text representation
    pub total_booked_tokens: u128,
}

impl EscrowStore {
    /// Creates a new EscrowStore with default values
    pub fn new() -> Self {
        Self {
            sale_status: SaleStatus::default(),
            booked_tokens: HashMap::new(),
            total_booked_tokens: 0,
        }
    }

    /// Get the current sale status
    pub fn get_sale_status(&self) -> &SaleStatus {
        &self.sale_status
    }

    /// Get a read-only view of the booked tokens
    pub fn get_booked_tokens(&self) -> &HashMap<Principal, u128> {
        &self.booked_tokens
    }

    pub fn get_participating_investors(&self) -> Vec<Principal> {
        self.booked_tokens.iter().map(|f| f.0.clone()).collect()
    }

    /// Get the total number of booked tokens
    pub fn get_total_booked_tokens(&self) -> u128 {
        self.total_booked_tokens
    }

    /// Book tokens for a specific owner
    pub fn book_tokens(&mut self, owner: Principal, quantity: u128) {
        let owner_key = owner.clone();
        let current_amount = self.booked_tokens.get(&owner_key).cloned().unwrap_or(0);
        self.booked_tokens.insert(owner_key, current_amount + quantity);
        self.total_booked_tokens += quantity;
    }

    /// Accept the sale
    pub fn accept_sale(&mut self) {
        self.sale_status = SaleStatus::Accepted;
    }

    /// Reject the sale
    pub fn reject_sale(&mut self) {
        self.sale_status = SaleStatus::Rejected;
    }

    pub async fn icrc1_balance_of( token_ledger_canister_principal: Principal ,arg: Icrc1Account) -> Result<u128, String> {

        let response = call(token_ledger_canister_principal, "icrc1_balance_of", (arg, )).await;
        
            match response {
                Ok((bal, )) => Ok(bal),
                Err((e, err_msg)) => Err(format!("Failed to grant permission: {e:?} {}", err_msg)),
            }
    }

}
