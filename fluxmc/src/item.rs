mod material;
mod meta;

pub use material::*;
pub use meta::*;
use serde::Serialize;

use crate::id::Identifier;

#[derive(Debug, Clone, Serialize)]
pub struct ItemStack {
    id: Identifier,
    #[serde(rename = "tag")]
    meta: ItemMetaTag,
    count: i8,
}

#[derive(Debug, Copy, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Feet,
    Legs,
    Chest,
    Head,
}
