mod misc;

use bitflags::bitflags;

use serde::Serialize;

use crate::{
    attr::AttributeModifier, effect::PotionContainerMeta, id::Identifier, text::StrComponent,
};

use super::{
    book::{BookAndQuillMeta, WrittenBookMeta},
    ench::{EnchantedBookMeta, Enchantment},
    FireworkExplosion,
};

pub use misc::*;

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct ItemMetaTag {
    damage: i32,
    unbreakable: bool,
    can_destroy: Vec<Identifier>,
    custom_model_data: i32,
    attribute_modifiers: Vec<AttributeModifier>,
    #[serde(rename = "display")]
    display: ItemDisplay,
    hide_flags: HideFlags,
    enchantments: Vec<Enchantment>,
    repair_cost: i32,
    #[serde(flatten)]
    _container: ItemMeta,
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct ItemDisplay {
    #[serde(rename = "color")]
    leather_color: Option<i32>,
    name: StrComponent,
    lore: Vec<StrComponent>,
}

bitflags! {
    struct HideFlags: u32 {
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
}
