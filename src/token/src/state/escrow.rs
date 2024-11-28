use candid::{CandidType, Deserialize,  Principal};
use ic_cdk::api::call::call;
use serde::Serialize;
use std::collections::HashMap;

use crate::{state::subaccount::{Subaccount, AccountIdentifier}, Icrc1Account};

use super::metadata::Metadata;

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
    pub booked_tokens: HashMap<Principal, u128>, // Using `String` for Principal text representation
    pub total_booked_tokens: u128,
}

impl EscrowStore {
    /// Creates a new EscrowStore with default values
    pub fn default() -> Self {
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

        ic_cdk::println!("Balance args: {} {arg:?}", arg.owner.to_text());

        let response = call(token_ledger_canister_principal, "icrc1_balance_of", (arg, )).await;
        
            match response {
                Ok((bal, )) => Ok(bal),
                Err((e, err_msg)) => Err(format!("Failed to grant permission: {e:?} {}", err_msg)),
            }
    }

    pub async fn refund_from_escrow(&self, invester: &Principal, metadata: Metadata) -> Result<RefundResult, String> {
        let icp_ledger_index = metadata.index;
    let icp_ledger = metadata.token;

    let escrow_subaccount: Subaccount = invester.into();
    let escrow_account_id = AccountIdentifier::from_principal(ic_cdk::id(), Some(escrow_subaccount));

    // Query transactions
    let (index_query_result,): (GetTransactions, ) = call(icp_ledger_index, "get_account_transactions", (GetAccountTransactionsArgs {
        account: Icrc1Account {
            owner: ic_cdk::id(),
            subaccount: Some(escrow_subaccount.to_vec()),
        },
        start: None,
        max_results: 5,
    }, )).await.map_err(|(c,e)| format!("Failed to get account transactions: {c:?} {e} "))?;


    // Validate query result
    let escrow_balance = index_query_result.balance;
    let transactions = index_query_result
        .transactions
        .into_iter()
        .filter_map(|txn| txn.transaction.transfer)
        .collect::<Vec<_>>();

    let refund_account_id = transactions
        .iter()
        .find(|txn| AccountIdentifier::from_principal(txn.to.owner, txn.to.subaccount.clone().map(|f| Subaccount::try_from(f.as_slice()).ok()).flatten())  == escrow_account_id)
        .map(|txn| txn.from.clone());

    let refund_account_id = match refund_account_id {
        Some(account) => account,
        None => return Result::Err("Txn not found in ledger".to_string()),
    };

    // Calculate refund amount
    const TRANSFER_FEE: u64 = 10_000;
    let refund_amount = escrow_balance.saturating_sub(TRANSFER_FEE);
    if refund_amount == 0 {
        return Result::Ok(RefundResult {
            to: refund_account_id.owner.to_text(),
            amount: 0,
        });
    }

    // Transfer funds
    let _ : (Result<u64, TransferError1>,) = call(icp_ledger, "transfer", (GetAccountTransactionsArgs {
        account: Icrc1Account {
            owner: ic_cdk::id(),
            subaccount: Some(escrow_subaccount.to_vec()),
        },
        start: None,
        max_results: 5,
    }, )).await.map_err(|(c,e)| format!("Failed to transfer: {c:?} {e} "))?;


    Ok(RefundResult {
        to: refund_account_id.owner.to_text(),
        amount: refund_amount,
    })
       
    }
}

#[derive(CandidType, Deserialize)]
pub enum TransferError1 {
  TxTooOld{ allowed_window_nanos: u64 },
  BadFee{ expected_fee: Tokens },
  TxDuplicate{ duplicate_of: u64 },
  TxCreatedInFuture,
  InsufficientFunds{ balance: u64 },
}

#[derive(CandidType, Deserialize,)]
pub struct RefundResult {
    pub to: String,
    pub amount: u64,
}

#[derive( Clone, CandidType, Deserialize)]
struct GetAccountTransactionsArgs {
    account: Icrc1Account,
    start: Option<u64>,
    max_results: u64,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactions {
  pub balance: u64,
  pub transactions: Vec<TransactionWithId>,
  pub oldest_tx_id: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionWithId {
  pub id: u64,
  pub transaction: Transaction,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
  pub burn: Option<Burn>,
  pub kind: String,
  pub mint: Option<Mint>,
  pub approve: Option<Approve>,
  pub timestamp: u64,
  pub transfer: Option<Transfer>,
}

pub type Tokens = candid::Nat;
#[derive(CandidType, Deserialize)]
pub struct Burn {
  pub from: Icrc1Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub spender: Option<Icrc1Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Mint {
  pub to: Icrc1Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct Approve {
  pub fee: Option<candid::Nat>,
  pub from: Icrc1Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub expected_allowance: Option<candid::Nat>,
  pub expires_at: Option<u64>,
  pub spender: Icrc1Account,
}

#[derive(CandidType, Deserialize)]
pub struct Transfer {
  pub to: Icrc1Account,
  pub fee: Option<candid::Nat>,
  pub from: Icrc1Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub spender: Option<Icrc1Account>,
}