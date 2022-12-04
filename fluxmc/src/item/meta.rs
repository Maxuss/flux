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
    FireworkExplosion, Material,
};

pub use book::*;
pub use head::*;
pub use misc::*;

macro_rules! boilerplate_fields {
    ($(
            $setter:ident, $getter:ident as $field:ident -> $(&$ref:ty;)? $(!$ret:ty;)?
    )*) => {
        $(
            $(
                pub fn $getter(&self) -> Option<$ret> {
                    self.$field
                }
            )?

            $(
                pub fn $getter(&self) -> &Option<$ref> {
                    &self.$field
                }
            )?

            pub fn $setter(&mut self, value: $($ref)? $($ret)?) {
                self.$field = Some(value)
            }
        )*
    };
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ItemMetaTag {
    damage: Option<i32>,
    unbreakable: Option<bool>,
    can_destroy: Option<Vec<Identifier>>,
    custom_model_data: Option<i32>,
    attribute_modifiers: Option<Vec<AttributeModifier>>,
    #[serde(rename = "display")]
    display: Option<ItemDisplay>,
    hide_flags: Option<HideFlags>,
    enchantments: Option<Vec<Enchantment>>,
    repair_cost: Option<i32>,
    #[serde(flatten)]
    _container: ItemMeta,
}

impl ItemMetaTag {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn material_bound(material: Material) -> Self {
        let mut default = Self::default();
        let meta = match material {
            Material::EnchantedBook => ItemMeta::EnchantedBook(EnchantedBookMeta::default()),
            Material::Potion
            | Material::SplashPotion
            | Material::LingeringPotion
            | Material::TippedArrow => {
                ItemMeta::PotionContainer(PotionContainerMeta::new(Identifier::minecraft("null")))
            }
            Material::WritableBook => ItemMeta::BookAndQuill(BookAndQuillMeta::new(None)),
            Material::WrittenBook => ItemMeta::WrittenBook(WrittenBookMeta::default()),
            Material::CodBucket
            | Material::SalmonBucket
            | Material::PufferfishBucket
            | Material::TropicalFishBucket => ItemMeta::FishBucket(BucketMeta::new()),
            Material::Bundle => ItemMeta::Bundle(BundleMeta::new(None)),
            Material::Compass => ItemMeta::Compass(CompassMeta::default()),
            Material::Crossbow => ItemMeta::Crossbow(CrossbowMeta::uncharged()),
            Material::FireworkRocket => ItemMeta::Firework(FireworkMeta::default()),
            Material::FireworkStar => ItemMeta::FireworkStar(FireworkExplosion::default()),
            Material::Map | Material::FilledMap => ItemMeta::Map(MapMeta::default()),
            Material::PlayerHead => ItemMeta::Head(HeadMeta::default()),
            Material::SuspiciousStew => ItemMeta::SuspiciousStew(SuspiciousStewMeta::default()),
            _ => ItemMeta::EmptyMeta,
        };
        default._container = meta;
        default
    }

    boilerplate_fields! {
        set_damage, damage as damage -> !i32;
        set_unreakable, unbreakable as unbreakable -> !bool;
        set_destroying, can_destroy as can_destroy -> &Vec<Identifier>;
        set_custom_model_data, custom_model_data as custom_model_data -> !i32;
        set_attribute_modifiers, attribute_modifiers as attribute_modifiers -> &Vec<AttributeModifier>;
        set_display, display as display -> &ItemDisplay;
        hide_flags, hidden_flags as hide_flags -> !HideFlags;
        set_enchantments, enchantments as enchantments -> &Vec<Enchantment>;
        set_repair_cost, repair_cost as repair_cost -> !i32;
    }

    pub fn meta(&self) -> &ItemMeta {
        &self._container
    }

    pub fn set_meta(&mut self, meta: ItemMeta) {
        self._container = meta
    }

    pub fn with_meta<F: FnOnce(&mut ItemMeta)>(&mut self, modifier: F) {
        modifier(&mut self._container)
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

macro_rules! meta_enum {
    (
        $(
            $variant:ident($container:ty) => $getter:ident
        ),*
    ) => {
        #[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd)]
        #[serde(untagged)]
        pub enum ItemMeta {
            #[default]
            EmptyMeta,
            $(
                $variant($container)
            ),*
        }

        impl ItemMeta {
            pub fn is_empty(&self) -> bool {
                matches!(self, ItemMeta::EmptyMeta)
            }

            $(
                pub fn $getter(&mut self) -> Option<&mut $container> {
                    match self {
                        Self::$variant(value) => Some(value),
                        _ => None
                    }
                }
            )*
        }
    };
}

meta_enum! {
    EnchantedBook(EnchantedBookMeta) => as_enchanted_book,
    PotionContainer(PotionContainerMeta) => as_potion_container,
    BookAndQuill(BookAndQuillMeta) => as_book_and_quill,
    WrittenBook(WrittenBookMeta) => as_written_book,
    FishBucket(BucketMeta) => as_fish_bucket,
    Bundle(BundleMeta) => as_bundle,
    Compass(CompassMeta) => as_compass,
    Crossbow(CrossbowMeta) => as_crossbow,
    Firework(FireworkMeta) => as_firework,
    FireworkStar(FireworkExplosion) => as_firework_star,
    Map(MapMeta) => as_map,
    Head(HeadMeta) => as_head,
    SuspiciousStew(SuspiciousStewMeta) => as_suspicous_stew
}
