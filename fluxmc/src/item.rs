pub mod ench;
mod firework;
mod material;
mod meta;

use std::fmt::Display;

pub use firework::*;
use flux_nbt::ser::to_snbt;
pub use material::*;
pub use meta::*;
use serde::Serialize;

use crate::id::Identifier;

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct ItemStack {
    id: Identifier,
    #[serde(rename = "tag")]
    pub meta: ItemMetaTag,
    count: i8,
}

impl ItemStack {
    pub fn new(material: Material) -> Self {
        Self {
            id: material.into(),
            meta: ItemMetaTag::default(),
            count: 1,
        }
    }

    pub fn id(&self) -> &Identifier {
        &self.id
    }

    pub fn set_id(&mut self, id: Identifier) {
        self.id = id
    }

    pub fn count(&self) -> i8 {
        self.count
    }

    pub fn set_count(&mut self, count: i8) {
        self.count = count
    }
}

impl Display for ItemStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{} {}",
            self.id,
            to_snbt(&self.meta).unwrap(),
            self.count
        ))
    }
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
