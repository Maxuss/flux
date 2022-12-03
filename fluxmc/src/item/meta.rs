mod book;
mod head;
mod misc;

use bitflags::bitflags;

use lobsterchat::component::Component;
use serde::Serialize;

use crate::{
    attr::AttributeModifier, effect::PotionContainerMeta, id::Identifier, text::StrComponent,
};

use super::{
    ench::{EnchantedBookMeta, Enchantment},
    FireworkExplosion,
};

pub use book::*;
pub use head::*;
pub use misc::*;

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ItemMetaTag {
    pub damage: Option<i32>,
    pub unbreakable: Option<bool>,
    pub can_destroy: Option<Vec<Identifier>>,
    pub custom_model_data: Option<i32>,
    pub attribute_modifiers: Option<Vec<AttributeModifier>>,
    #[serde(rename = "display")]
    pub display: Option<ItemDisplay>,
    pub hide_flags: Option<HideFlags>,
    pub enchantments: Option<Vec<Enchantment>>,
    pub repair_cost: Option<i32>,
    #[serde(flatten)]
    _container: ItemMeta,
}

impl ItemMetaTag {
    pub fn empty() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct ItemDisplay {
    #[serde(rename = "color")]
    leather_color: Option<i32>,
    name: StrComponent,
    lore: Vec<StrComponent>,
}

impl ItemDisplay {
    pub fn new() -> Self {
        Self {
            leather_color: None,
            name: Component::default().into(),
            lore: Vec::with_capacity(4),
        }
    }

    pub fn name(&self) -> Component {
        self.name.value().to_owned()
    }

    pub fn lore(&self) -> Vec<Component> {
        self.lore
            .iter()
            .map(|each| each.value().to_owned())
            .collect()
    }

    pub fn set_name(&mut self, name: Component) {
        self.name = name.into()
    }

    pub fn set_lore(&mut self, lore: Vec<Component>) {
        self.lore = lore.into_iter().map(|each| each.into()).collect()
    }
}

impl Default for ItemDisplay {
    fn default() -> Self {
        Self::new()
    }
}

bitflags! {
    pub struct HideFlags: u32 {
        const ENCHANTMENTS =        0b0000001;
        const ATTRIBUTE_MODIFIERS = 0b0000010;
        const UNBREAKABLE =         0b0000100;
        const CAN_DESTROY =         0b0001000;
        const CAN_PLACE_ON =        0b0010000;
        const MISC =                0b0100000;
        const DYED =                0b1000000;
    }
}

impl Serialize for HideFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.bits)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum ItemMeta {
    #[default]
    EmptyMeta,
    EnchantedBook(EnchantedBookMeta),
    PotionContainer(PotionContainerMeta),
    BookAndQuill(BookAndQuillMeta),
    WrittenBook(WrittenBookMeta),
    FishBucket(BucketMeta),
    Bundle(BundleMeta),
    Compass(CompassMeta),
    Crossbow(CrossbowMeta),
    Firework(FireworkMeta),
    FireworkStar(FireworkExplosion),
    Map(MapMeta),
    Head(HeadMeta),
    SuspiciousStew(SuspiciousStewMeta),
}
