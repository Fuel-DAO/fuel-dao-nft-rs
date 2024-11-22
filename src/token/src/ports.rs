use candid::{Nat, Principal};
use ic_cdk::caller;
use crate::validations::{check_collection_owner,check_not_anonymous};
use crate::BookTokensArg;
use crate::{state::{escrow::SaleStatus, models::{GetEscrowAccountRet, GetMetadataRet}}, STATE};
use ic_cdk_macros::*;


#[update(guard = "check_collection_owner")]
pub async fn change_ownership( arg0: Principal) -> Result<Nat, String> {
    let   f  =  STATE.with_borrow_mut( |f|  f.clone() );
    f.change_ownership(arg0).await 
}


#[update(guard = "check_not_anonymous")]
pub async fn book_tokens( arg: BookTokensArg) -> Result<bool, String> {
    let   f  =  STATE.with_borrow( |f|  f.clone() );
    let qunatity =  arg.quantity.clone();
    let res = f.book_tokens(arg).await?;

    STATE.with(|f|{  f.borrow_mut().escrow.book_tokens(caller(), qunatity.into());  } );
    Ok(res)
}

#[query]
pub async fn get_booked_tokens( arg0: Option<Principal>) -> u128 {
    STATE.with( |f|  f.borrow().clone() )
    .get_booked_tokens(arg0).await 
}


#[query]
pub async fn get_escrow_account() -> Result<GetEscrowAccountRet, String> {
    STATE.with( |f|  f.borrow().clone() )
    .get_escrow_account().await 
}

#[query]
pub async fn get_metadata() -> Result<GetMetadataRet, String> {
    STATE.with( |f|  f.borrow().clone() )
    .get_metadata().await 
}

#[query]
pub async fn get_participating_investors() -> Vec<Principal> {
    STATE.with( |f|  f.borrow().clone() )
    .get_participating_investors().await 
}


#[query]
pub async fn get_sale_status() -> SaleStatus {
    STATE.with( |f|  f.borrow().clone() )
    .get_sale_status().await 
}


#[query]
pub async fn get_total_booked_tokens() -> u128 {
    STATE.with( |f|  f.borrow().clone() )
    .get_total_booked_tokens().await 
}
