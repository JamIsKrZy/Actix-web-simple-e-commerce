



// region:    --- Schemas

use rust_decimal::Decimal;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct BundleItem{
    pub product_id: i32,
    pub bundle_id: i32,
    pub quantity: i32
}

pub struct AddBundle{
    pub name: String,
    pub items: Vec<BundleItem>,
    pub price: Decimal
}


pub struct ForAdminBundleList{
    pub name: String,
    pub items: Vec<BundleItem>,
    pub price: Decimal
}

// endregion: --- Schemas