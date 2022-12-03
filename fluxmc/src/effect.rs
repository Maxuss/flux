use std::fmt::Display;

use convert_case::{Case, Casing};
use serde::Serialize;

use crate::id::Identifier;

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct PotionContainerMeta {
    potion: Identifier,
    custom_potion_effects: Vec<CustomPotionEffect>,
    custom_potion_color: Option<u32>,
}

impl PotionContainerMeta {
    pub fn new(display_potion: Identifier) -> Self {
        Self {
            potion: display_potion,
            custom_potion_effects: vec![],
            custom_potion_color: None,
        }
    }

    pub fn color(&self) -> Option<u32> {
        self.custom_potion_color
    }

    pub fn effects(&self) -> &Vec<CustomPotionEffect> {
        &self.custom_potion_effects
    }

    pub fn display_effect(&self) -> &Identifier {
        &self.potion
    }

    pub fn add_effect(&mut self, effect: CustomPotionEffect) {
        self.custom_potion_effects.push(effect)
    }

    pub fn set_color(&mut self, color: u32) {
        self.custom_potion_color = Some(color)
    }

    pub fn set_display_effect(&mut self, effect: Identifier) {
        self.potion = effect
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct CustomPotionEffect {
    id: EffectKind,
    amplifier: i8,
    duration: Option<u32>,
    ambient: Option<bool>,
    show_particles: Option<bool>,
    show_icon: Option<bool>,
}

impl CustomPotionEffect {
    pub fn new(kind: EffectKind, level: i8) -> Self {
        Self {
            id: kind,
            amplifier: level,
            duration: None,
            ambient: None,
            show_particles: None,
            show_icon: None,
        }
    }

    pub fn id(&self) -> &EffectKind {
        &self.id
    }

    pub fn amplifier(&self) -> i8 {
        self.amplifier
    }

    pub fn duration(&self) -> Option<u32> {
        self.duration
    }

    pub fn ambient(&self) -> Option<bool> {
        self.ambient
    }

    pub fn show_particles(&self) -> Option<bool> {
        self.show_particles
    }

    pub fn show_icon(&self) -> Option<bool> {
        self.show_icon
    }

    pub fn set_duration(&mut self, ticks: u32) {
        self.duration = Some(ticks)
    }

    pub fn set_ambient(&mut self, ambient: bool) {
        self.ambient = Some(ambient)
    }

    pub fn set_particles(&mut self, toggle: bool) {
        self.show_particles = Some(toggle)
    }

    pub fn set_icon(&mut self, toggle: bool) {
        self.show_icon = Some(toggle)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum EffectKind {
    Speed,
    Slowness,
    Haste,
    MiningFatigue,
    Strength,
    InstantHealth,
    InstantDamage,
    JumpBoost,
    Nausea,
    Regeneration,
    Resistance,
    FireResistance,
    WaterBreathing,
    Invisibility,
    Blindness,
    NightVision,
    Hunger,
    Weakness,
    Poison,
    Wither,
    HealthBoost,
    Absorption,
    Saturation,
    Glowing,
    Levitation,
    Luck,
    Unluck,
    SlowFalling,
    ConduitPower,
    DolphinGrace,
    BadOmen,
    HeroOfTheVillage,
    Darkness,
    Custom(Identifier),
}

impl From<EffectKind> for Identifier {
    fn from(value: EffectKind) -> Self {
        match value {
            EffectKind::Custom(custom) => custom,
            _ => Identifier::minecraft(format!("{value:?}").to_case(Case::Snake)),
        }
    }
}

impl From<EffectKind> for u32 {
    fn from(value: EffectKind) -> Self {
        match value {
            EffectKind::Speed => 1,
            EffectKind::Slowness => 2,
            EffectKind::Haste => 3,
            EffectKind::MiningFatigue => 4,
            EffectKind::Strength => 5,
            EffectKind::InstantHealth => 6,
            EffectKind::InstantDamage => 7,
            EffectKind::JumpBoost => 8,
            EffectKind::Nausea => 9,
            EffectKind::Regeneration => 10,
            EffectKind::Resistance => 11,
            EffectKind::FireResistance => 12,
            EffectKind::WaterBreathing => 13,
            EffectKind::Invisibility => 14,
            EffectKind::Blindness => 15,
            EffectKind::NightVision => 16,
            EffectKind::Hunger => 17,
            EffectKind::Weakness => 18,
            EffectKind::Poison => 19,
            EffectKind::Wither => 20,
            EffectKind::HealthBoost => 21,
            EffectKind::Absorption => 22,
            EffectKind::Saturation => 23,
            EffectKind::Glowing => 24,
            EffectKind::Levitation => 25,
            EffectKind::Luck => 26,
            EffectKind::Unluck => 27,
            EffectKind::SlowFalling => 28,
            EffectKind::ConduitPower => 29,
            EffectKind::DolphinGrace => 30,
            EffectKind::BadOmen => 31,
            EffectKind::HeroOfTheVillage => 32,
            EffectKind::Darkness => 33,
            EffectKind::Custom(_) => 0,
        }
    }
}

impl Display for EffectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", Identifier::from(self.to_owned())))
    }
}

impl Serialize for EffectKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            EffectKind::Custom(custom) => serializer.serialize_str(&custom.to_string()),
            _ => serializer.serialize_u32(self.to_owned().into()),
        }
    }
}
