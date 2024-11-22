
use candid::{CandidType, Deserialize,};
use super::metadata::Metadata;
use super::escrow::EscrowStore;
use super::transactions::TxnIndexStore;

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct State {
    pub metadata: Option<MetaDataState>, 
    pub escrow: EscrowStore,
    pub transactions: TxnIndexStore,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct MetaDataState {
    pub metadata: Metadata, 
    pub total_supply: u64
}


