#![allow(dead_code, unused_imports)]
use super::{
    account_identifier::AccountIdentifier,
    escrow::{EscrowStore, SaleStatus},
    models::*,
    subaccount::SubAccount,
    State,
};
use candid::{self, CandidType, Decode, Deserialize, Encode, Nat, Principal};
use ic_cdk::{api::call::CallResult, caller};

impl State {
    pub async fn accept_sale(&self) -> CallResult<(AcceptSaleRet,)> {
        ic_cdk::call(Principal::anonymous(), "accept_sale", ()).await
    }
    pub async fn accept_sale_individual(
        &self,
        arg0: Principal,
    ) -> CallResult<(AcceptSaleIndividualRet,)> {
        ic_cdk::call(Principal::anonymous(), "accept_sale_individual", (arg0,)).await
    }

    /// Should not be anonymous
    pub async fn book_tokens(&self, arg: BookTokensArg) -> Result<bool, String> {
        let principal = caller();
        let metadata = self.get_metadata().await?; // Assume this retrieves the Metadata struct

        let mut escrow_store = self
            .escrow.clone()
            ; // Assume this retrieves the EscrowStore instance

        if arg.quantity <= 0 {
            return Err("Quantity should be at least 1.".to_string());
        }

        if escrow_store.sale_status != SaleStatus::Live {
            return Err("Sale not live.".to_string());
        }

        let subaccount = SubAccount::derive_subaccount(&principal);
        let icp_ledger = metadata.token;

        let escrow_balance = EscrowStore::icrc1_balance_of(
            icp_ledger,
            Icrc1Account {
                owner: principal,
                subaccount: Some(subaccount.bytes.to_vec()),
            },
        )
        .await?;

        let total_invested_count = escrow_store.get_booked_tokens()
            .get(&principal)
            .cloned()
            .unwrap_or_else(|| 0);

        let total_cost = ((&total_invested_count + &(arg.quantity as u128)) as f64) * &(metadata.price + 10_000.0);

        if (escrow_balance as f64) < total_cost {
            return Err("Invalid balance in escrow.".to_string());
        }

        if &escrow_store.total_booked_tokens + &(arg.quantity as u128) > metadata.supply_cap {
            return Err("Supply cap reached.".to_string());
        }

        escrow_store.book_tokens(principal, arg.quantity.into());

        Ok(true)
    }

    pub async fn change_ownership(&self, arg0: Principal) -> Result<Nat, String> {
        let canister = self
            .metadata
            .clone()
            .map(|f| f.metadata.asset_canister)
            .ok_or("Metadata not set".to_string())?;

        let current_user = self
            .metadata
            .clone()
            .map(|f| f.metadata.collection_owner)
            .ok_or("Metadata not set".to_string())?;

        crate::permissions::grant_asset_edit_perms(canister, arg0).await?;

        crate::permissions::revoke_asset_edit_perms(canister, current_user).await?;

        Ok(self.transactions.index().clone())
    }

    pub async fn get_booked_tokens(&self, arg0: Option<Principal>) -> u128 {
        let user = arg0.unwrap_or(caller());

        self.escrow
            .clone()
            .get_booked_tokens().get(&user).cloned()
            .unwrap_or(0)
    }

    pub async fn get_escrow_account(&self) -> Result<GetEscrowAccountRet, String> {
        let principal = ic_cdk::api::id();
        let subaccount = SubAccount::derive_subaccount(&ic_cdk::caller());

        let account_identifier = AccountIdentifier::from_principal(
            principal,
            Some(SubAccount::from_bytes(&subaccount.to_uint8_array())?),
        );

        Ok(GetEscrowAccountRet {
            account: GetEscrowAccountRetAccount {
                owner: principal,
                subaccount: subaccount.bytes,
            },
            account_id: account_identifier.to_hex(),
        })
    }

    pub async fn get_metadata(&self) -> Result<GetMetadataRet, String> {
        Ok(self
            .metadata
            .as_ref()
            .map(|f| f.metadata.with_supply(f.total_supply.into()))
            .clone()
            .ok_or("Init args not set".to_string())?)
    }

    pub async fn get_participating_investors(&self) -> Vec<Principal> {
        self.escrow.clone().get_participating_investors()
    }

    pub async fn get_sale_status(&self) -> SaleStatus {
        self.escrow
            .clone()
            .get_sale_status().clone()
    }

    pub async fn get_total_booked_tokens(&self) -> u128 {
        self.escrow
            .clone()
            .get_total_booked_tokens().clone()
    }

    pub async fn icrc_61_supported_standards(
        &self,
    ) -> CallResult<(Vec<Icrc61SupportedStandardsRetItem>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc61_supported_standards", ()).await
    }
    pub async fn icrc_7_atomic_batch_transfers(&self) -> CallResult<(Option<bool>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_atomic_batch_transfers", ()).await
    }
    pub async fn icrc_7_balance_of(
        &self,
        arg0: Vec<Icrc7BalanceOfArgItem>,
    ) -> CallResult<(Vec<candid::Nat>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_balance_of", (arg0,)).await
    }
    pub async fn icrc_7_collection_metadata(
        &self,
    ) -> CallResult<(Vec<(String, Icrc7CollectionMetadataRetItem1)>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_collection_metadata", ()).await
    }
    pub async fn icrc_7_description(&self) -> CallResult<(Option<String>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_description", ()).await
    }
    pub async fn icrc_7_logo(&self) -> CallResult<(Option<String>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_logo", ()).await
    }
    pub async fn icrc_7_max_default_take_value(&self) -> CallResult<(Option<candid::Nat>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_max_default_take_value", ()).await
    }
    pub async fn icrc_7_max_memo_size(&self) -> CallResult<(Option<candid::Nat>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_max_memo_size", ()).await
    }
    pub async fn icrc_7_max_query_batch_size(&self) -> CallResult<(Option<candid::Nat>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_max_query_batch_size", ()).await
    }
    pub async fn icrc_7_max_take_value(&self) -> CallResult<(Option<candid::Nat>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_max_take_value", ()).await
    }
    pub async fn icrc_7_max_update_batch_size(&self) -> CallResult<(Option<candid::Nat>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_max_update_batch_size", ()).await
    }
    pub async fn icrc_7_name(&self) -> CallResult<(String,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_name", ()).await
    }
    pub async fn icrc_7_owner_of(
        &self,
        arg0: Vec<candid::Nat>,
    ) -> CallResult<(Vec<Option<Icrc7OwnerOfRetItemInner>>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_owner_of", (arg0,)).await
    }
    pub async fn icrc_7_permitted_drift(&self) -> CallResult<(Option<candid::Nat>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_permitted_drift", ()).await
    }
    pub async fn icrc_7_supply_cap(&self) -> CallResult<(Option<candid::Nat>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_supply_cap", ()).await
    }
    pub async fn icrc_7_symbol(&self) -> CallResult<(String,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_symbol", ()).await
    }
    pub async fn icrc_7_token_metadata(
        &self,
        arg0: Vec<candid::Nat>,
    ) -> CallResult<(Vec<Option<Vec<(String, Icrc7TokenMetadataRetItemInnerItem1)>>>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_token_metadata", (arg0,)).await
    }
    pub async fn icrc_7_tokens(
        &self,
        arg0: Option<candid::Nat>,
        arg1: Option<candid::Nat>,
    ) -> CallResult<(Vec<candid::Nat>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_tokens", (arg0, arg1)).await
    }
    pub async fn icrc_7_tokens_of(
        &self,
        arg0: Icrc7TokensOfArg,
        arg1: Option<candid::Nat>,
        arg2: Option<candid::Nat>,
    ) -> CallResult<(Vec<candid::Nat>,)> {
        ic_cdk::call(
            Principal::anonymous(),
            "icrc7_tokens_of",
            (arg0, arg1, arg2),
        )
        .await
    }
    pub async fn icrc_7_total_supply(&self) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_total_supply", ()).await
    }
    pub async fn icrc_7_transfer(
        &self,
        arg0: Vec<Icrc7TransferArgItem>,
    ) -> CallResult<(Vec<Option<Icrc7TransferRetItemInner>>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_transfer", (arg0,)).await
    }
    pub async fn icrc_7_tx_window(&self) -> CallResult<(Option<candid::Nat>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_tx_window", ()).await
    }
    pub async fn refund_excess_after_sale(
        &self,
        arg0: Principal,
    ) -> CallResult<(RefundExcessAfterSaleRet,)> {
        ic_cdk::call(Principal::anonymous(), "refund_excess_after_sale", (arg0,)).await
    }

    pub async fn reject_sale(&self) -> CallResult<(RejectSaleRet,)> {
        ic_cdk::call(Principal::anonymous(), "reject_sale", ()).await
    }
    pub async fn reject_sale_individual(
        &self,
        arg0: Principal,
    ) -> CallResult<(RejectSaleIndividualRet,)> {
        ic_cdk::call(Principal::anonymous(), "reject_sale_individual", (arg0,)).await
    }

    pub async fn update_metadata(
        &self,
        arg0: UpdateMetadataArg,
    ) -> CallResult<(UpdateMetadataRet,)> {
        ic_cdk::call(Principal::anonymous(), "update_metadata", (arg0,)).await
    }
}
