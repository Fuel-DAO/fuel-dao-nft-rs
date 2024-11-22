use candid::{CandidType, Deserialize, Nat, Principal};

use super::models::GetMetadataRet;



#[derive(CandidType, Deserialize, Clone)]
pub struct Metadata {
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



impl Metadata {
    pub fn with_supply(&self, total_supply: Nat) -> GetMetadataRet {
        GetMetadataRet {
            weight: self.weight,
            drive_type: self.drive_type.clone(),
            purchase_price: self.purchase_price.clone(),
            token: self.token,
            documents: self.documents.clone(),
            supply_cap: self.supply_cap.clone(),
            displays: self.displays.clone(),
            seating: self.seating.clone(),
            cargo: self.cargo,
            logo: self.logo.clone(),
            name: self.name.clone(),
            overall_height: self.overall_height,
            description: self.description.clone(),
            overall_width: self.overall_width,
            track_front: self.track_front,
            collection_owner: self.collection_owner,
            asset_canister: self.asset_canister,
            ground_clearance: self.ground_clearance,
            key_features: self.key_features.clone(),
            range_per_charge: self.range_per_charge,
            track_rear: self.track_rear,
            acceleration: self.acceleration.clone(),
            charging_speed: self.charging_speed.clone(),
            wheels: self.wheels,
            brochure_url: self.brochure_url.clone(),
            index: self.index,
            price: self.price.clone(),
            battery: self.battery.clone(),
            overall_length: self.overall_length,
            total_supply,
            symbol: self.symbol.clone(),
            treasury: self.treasury,
            images: self.images.clone(),
        }
    }
}