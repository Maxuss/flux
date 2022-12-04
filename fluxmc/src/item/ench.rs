use std::fmt::Display;

use convert_case::{Case, Casing};
use serde::Serialize;

use crate::id::Identifier;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Enchantment {
    id: EnchantKind,
    lvl: u16,
}

impl Enchantment {
    pub fn new(kind: EnchantKind, lvl: u16) -> Self {
        Self { id: kind, lvl }
    }

    pub fn kind(&self) -> &EnchantKind {
        &self.id
    }

    pub fn level(&self) -> u16 {
        self.lvl
    }

    pub fn kind_mut(&mut self) -> &mut EnchantKind {
        &mut self.id
    }

    pub fn level_mut(&mut self) -> &mut u16 {
        &mut self.lvl
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum EnchantKind {
    AquaAffinity,
    BaneOfArthropods,
    BlastProtection,
    Channeling,
    CurseOfBinding,
    CurseOfVanishing,
    DepthStrider,
    Efficiency,
    FeatherFalling,
    FireAspect,
    FireProtection,
    Flame,
    Fortune,
    FrostWalker,
    Impaling,
    Infinity,
    Knockback,
    Looting,
    Loyalty,
    LuckOfTheSea,
    Lure,
    Mending,
    Multishot,
    Piercing,
    Power,
    ProjectileProtection,
    Protection,
    Punch,
    QuickCharge,
    Respiration,
    Riptide,
    Sharpness,
    SilkTouch,
    Smite,
    SoulSpeed,
    SweepingEdge,
    SwiftSneak,
    Thorns,
    Unbreaking,
    Custom(Identifier),
}

impl From<EnchantKind> for Identifier {
    fn from(value: EnchantKind) -> Self {
        match value {
            EnchantKind::Custom(custom) => custom,
            _ => Identifier::minecraft(format!("{value:?}").to_case(Case::Snake)),
        }
    }
}

impl Display for EnchantKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", Identifier::from(self.to_owned())))
    }
}

impl Serialize for EnchantKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Default, Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct EnchantedBookMeta {
    stored_enchantments: Vec<Enchantment>,
}

impl EnchantedBookMeta {
    pub fn new() -> Self {
        Self {
            stored_enchantments: Vec::with_capacity(2),
        }
    }

    pub fn new_full(enchs: Vec<Enchantment>) -> Self {
        Self {
            stored_enchantments: enchs,
        }
    }

    pub fn stored_enchantments(&self) -> &Vec<Enchantment> {
        &self.stored_enchantments
    }

    pub fn add_enchantment(&mut self, ench: Enchantment) {
        self.stored_enchantments.push(ench)
    }
}
