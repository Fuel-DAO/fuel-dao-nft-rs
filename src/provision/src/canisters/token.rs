use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::{
    main::{create_canister, install_code, CreateCanisterArgument, InstallCodeArgument},
    provisional::CanisterSettings,
};

use crate::CollectionRequest;

#[derive(CandidType, Deserialize)]
pub enum TokenCanisterArgs {
  Upgrade,
  Init{
    metadata: CollectionMetadata
  },
}

#[derive(CandidType, Deserialize)]
pub struct CollectionMetadata {
    pub weight: f64,
    pub drive_type: String,
    pub purchase_price: u128,
    pub token: Principal,
    pub documents: Vec<(String, String)>,
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
    pub symbol: String,
    pub treasury: Principal,
    pub images: Vec<String>,
}


impl CollectionRequest {
    pub fn into_metadata(&self, collection_owner:  Principal , asset_canister: Principal) -> CollectionMetadata {
      CollectionMetadata {
          weight: self.weight,
          collection_owner,
          asset_canister,
          drive_type: self.drive_type.clone(),
          purchase_price: self.purchase_price,
          token: self.token,
          documents: self.documents.clone(),
          supply_cap: self.supply_cap,
          displays: self.displays.clone(),
          seating: self.seating.clone(),
          cargo: self.cargo,
          logo: self.logo.clone(),
          name: self.name.clone(),
          overall_height: self.overall_height,
          description: self.description.clone(),
          overall_width: self.overall_width,
          track_front: self.track_front,
          ground_clearance: self.ground_clearance,
          key_features: self.key_features.clone(),
          range_per_charge: self.range_per_charge,
          track_rear: self.track_rear,
          acceleration: self.acceleration.clone(),
          charging_speed: self.charging_speed.clone(),
          wheels: self.wheels,
          brochure_url: self.brochure_url.clone(),
          index: self.index,
          price: self.price,
          battery: self.battery.clone(),
          overall_length: self.overall_length,
          symbol: self.symbol.clone(),
          treasury: self.treasury,
          images: self.images.clone(),
      }
  }
  }
  



pub async fn deploy_token(wasm: Vec<u8>, request: CollectionMetadata) -> Result<Principal, String> {
    // Step 1: Create a new canister with updated CanisterSettings
    let canister_id = match create_canister(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(vec![ic_cdk::api::id()]),
                ..Default::default()
            }),
        },
        /* 14_000_000_000, */ 7_692_307_692 + 6_153_894_868 + 3_076_923_077,
    )
    .await
    {
        Ok(response) => response.0.canister_id,
        Err((_, err_msg)) => return Err(format!("Failed to create token canister: {}", err_msg)),
    };

    // Step 2: Install chunked code on the created canister
    let install_code_arg = InstallCodeArgument {
        mode: ic_cdk::api::management_canister::main::CanisterInstallMode::Install,
        canister_id,
        wasm_module: wasm,
        arg: candid::encode_args((TokenCanisterArgs::Init { metadata: request } ,)).unwrap(),
    };

    if let Err((e, err_msg)) = install_code(install_code_arg).await {
        return Err(format!("Failed to install code into Token wasm: {e:?} {}", err_msg));
    }

    // Step 3: Return the created canister ID
    Ok(canister_id)
}
