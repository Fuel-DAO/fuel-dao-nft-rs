// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Nat, Principal};
use ic_cdk::api::call::CallResult as CallResult;
use serde::Serialize;



#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Icrc1Account {
  pub owner: Principal, 
  pub subaccount: Option<Vec<u8>>
}


#[derive(CandidType, Deserialize, Clone)]
pub enum Init {
  Upgrade,
  Init{
    weight: f64,
    drive_type: String,
    purchase_price: candid::Nat,
    token: Principal,
    documents: Vec<(String,String,)>,
    supply_cap: candid::Nat,
    displays: String,
    seating: String,
    cargo: f64,
    logo: String,
    name: String,
    overall_height: f64,
    description: String,
    overall_width: f64,
    track_front: f64,
    collection_owner: Principal,
    asset_canister: Principal,
    ground_clearance: f64,
    key_features: Vec<String>,
    range_per_charge: f64,
    track_rear: f64,
    acceleration: String,
    charging_speed: String,
    wheels: f64,
    brochure_url: String,
    index: Principal,
    price: candid::Nat,
    battery: String,
    overall_length: f64,
    symbol: String,
    treasury: Principal,
    images: Vec<String>,
  },
}

#[derive(CandidType, Deserialize, Clone)]
pub enum AcceptSaleRet { Ok(bool), Err(String) }

#[derive(CandidType, Deserialize, Clone)]
pub enum AcceptSaleIndividualRet { Ok(bool), Err(String) }

#[derive(CandidType, Deserialize, Clone)]
pub struct BookTokensArg { pub quantity: u32 }


#[derive(CandidType, Deserialize, Clone)]
pub enum ChangeOwnershipRet { Ok(candid::Nat), Err(String) }

#[derive(CandidType, Deserialize, Clone)]
pub struct GetEscrowAccountRetAccount {
  pub owner: Principal,
  pub subaccount: [u8; 32],
}

#[derive(CandidType, Deserialize, Clone)]
pub struct GetEscrowAccountRet {
  pub account_id: String,
  pub account: GetEscrowAccountRetAccount,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct GetMetadataRet {
  pub weight: f64,
  pub drive_type: String,
  pub purchase_price: u128,
  pub token: Principal,
  pub documents: Vec<(String,String,)>,
  pub supply_cap: u128,
  pub displays: String,
  pub seating: String,
  pub cargo: f64,
  pub logo: String,
  pub name: String,
  pub overall_height: f64,
  pub description: String,
  pub overall_width: f64,
  pub track_front: f64,
  pub collection_owner: Principal,
  pub asset_canister: Principal,
  pub ground_clearance: f64,
  pub key_features: Vec<String>,
  pub range_per_charge: f64,
  pub track_rear: f64,
  pub acceleration: String,
  pub charging_speed: String,
  pub wheels: f64,
  pub brochure_url: String,
  pub index: Principal,
  pub price: f64,
  pub battery: String,
  pub overall_length: f64,
  pub total_supply: candid::Nat,
  pub symbol: String,
  pub treasury: Principal,
  pub images: Vec<String>,
}


#[derive(CandidType, Deserialize, Clone)]
pub struct Icrc61SupportedStandardsRetItem { pub url: String, pub name: String }

#[derive(CandidType, Deserialize, Clone)]
pub struct Icrc7BalanceOfArgItem {
  pub owner: Principal,
  pub subaccount: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum Icrc7CollectionMetadataRetItem1MapItem1 {
  Int(candid::Int),
  Nat(candid::Nat),
  Blob(Vec<u8>),
  Text(String),
}

#[derive(CandidType, Deserialize, Clone)]
pub enum Icrc7CollectionMetadataRetItem1ArrayItem {
  Int(candid::Int),
  Nat(candid::Nat),
  Blob(Vec<u8>),
  Text(String),
}

#[derive(CandidType, Deserialize, Clone)]
pub enum Icrc7CollectionMetadataRetItem1 {
  Int(candid::Int),
  Map(Vec<(String,Icrc7CollectionMetadataRetItem1MapItem1,)>),
  Nat(candid::Nat),
  Blob(Vec<u8>),
  Text(String),
  Array(Vec<Icrc7CollectionMetadataRetItem1ArrayItem>),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Icrc7OwnerOfRetItemInner {
  pub owner: Principal,
  pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum Icrc7TokenMetadataRetItemInnerItem1MapItem1 {
  Int(candid::Int),
  Nat(candid::Nat),
  Blob(Vec<u8>),
  Text(String),
}

#[derive(CandidType, Deserialize, Clone)]
pub enum Icrc7TokenMetadataRetItemInnerItem1ArrayItem {
  Int(candid::Int),
  Nat(candid::Nat),
  Blob(Vec<u8>),
  Text(String),
}

#[derive(CandidType, Deserialize, Clone)]
pub enum Icrc7TokenMetadataRetItemInnerItem1 {
  Int(candid::Int),
  Map(Vec<(String,Icrc7TokenMetadataRetItemInnerItem1MapItem1,)>),
  Nat(candid::Nat),
  Blob(Vec<u8>),
  Text(String),
  Array(Vec<Icrc7TokenMetadataRetItemInnerItem1ArrayItem>),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Icrc7TokensOfArg {
  pub owner: Principal,
  pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Icrc7TransferArgItemTo {
  pub owner: Principal,
  pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Icrc7TransferArgItem {
  pub to: Icrc7TransferArgItemTo,
  pub token_id: u32,
  pub memo: Option<Vec<u8>>,
  pub from_subaccount: Option<Vec<u8>>,
  pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum Icrc7TransferRetItemInnerErr {
  GenericError{ message: String, error_code: candid::Nat },
  Duplicate{ duplicate_of: candid::Nat },
  NonExistingTokenId,
  Unauthorized,
  CreatedInFuture{ ledger_time: u64 },
  InvalidRecipient,
  GenericBatchError{ message: String, error_code: candid::Nat },
  TooOld,
}

#[derive(CandidType, Deserialize, Clone)]
pub  struct TransferArgs {
 pub to: Icrc1Account,
 pub from_subaccount: Option<Vec<u8>>,
 pub fee: Option<u64>,
 pub memo: Option<String>,
 pub created_at_time: Option<u64>,
 pub amount: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum Icrc7TransferRetItemInner {
  Ok(u32),
  Err(Icrc7TransferRetItemInnerErr),
}

#[derive(CandidType, Deserialize, Clone)]
pub enum RefundExcessAfterSaleRet { Ok(bool), Err(String) }

#[derive(CandidType, Deserialize, Clone)]
pub enum RejectSaleRet { Ok(bool), Err(String) }

#[derive(CandidType, Deserialize, Clone)]
pub enum RejectSaleIndividualRet { Ok(bool), Err(String) }

#[derive(CandidType, Deserialize, Clone)]
pub struct UpdateMetadataArg {
  pub weight: Option<f64>,
  pub drive_type: Option<String>,
  pub purchase_price: Option<candid::Nat>,
  pub token: Option<Principal>,
  pub documents: Option<Vec<(String,String,)>>,
  pub supply_cap: Option<candid::Nat>,
  pub displays: Option<String>,
  pub seating: Option<String>,
  pub cargo: Option<f64>,
  pub logo: Option<String>,
  pub name: Option<String>,
  pub overall_height: Option<f64>,
  pub description: Option<String>,
  pub overall_width: Option<f64>,
  pub track_front: Option<f64>,
  pub asset_canister: Option<Principal>,
  pub ground_clearance: Option<f64>,
  pub key_features: Option<Vec<String>>,
  pub range_per_charge: Option<f64>,
  pub track_rear: Option<f64>,
  pub acceleration: Option<String>,
  pub charging_speed: Option<String>,
  pub wheels: Option<f64>,
  pub brochure_url: Option<String>,
  pub index: Option<Principal>,
  pub price: Option<candid::Nat>,
  pub battery: Option<String>,
  pub overall_length: Option<f64>,
  pub symbol: Option<String>,
  pub treasury: Option<Principal>,
  pub images: Option<Vec<String>>,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum UpdateMetadataRet { Ok(candid::Nat), Err(String) }
