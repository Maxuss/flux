use serde::Serialize;
use uuid::Uuid;

use crate::item::EquipmentSlot;

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct AttributeModifier {
    #[serde(rename = "AttributeName")]
    pub attribute: Attribute,
    #[serde(rename = "Amount")]
    pub amount: f64,
    #[serde(rename = "Operation")]
    pub operation: i32,
    #[serde(flatten)]
    additional_data: AdditionalAttributeData,
}

impl AttributeModifier {
    pub fn new(attribute: Attribute, operation: AttributeOperation, amount: f64) -> Self {
        Self {
            attribute,
            amount,
            operation: operation.into(),
            additional_data: AdditionalAttributeData::None,
        }
    }

    pub fn new_slot(
        attribute: Attribute,
        operation: AttributeOperation,
        amount: f64,
        slot: EquipmentSlot,
    ) -> Self {
        Self {
            attribute,
            amount,
            operation: operation.into(),
            additional_data: AdditionalAttributeData::Full {
                slot,
                uuid: Uuid::new_v4(),
            },
        }
    }

    pub fn new_full(
        attribute: Attribute,
        operation: AttributeOperation,
        amount: f64,
        slot: EquipmentSlot,
        uuid: Uuid,
    ) -> Self {
        Self {
            attribute,
            amount,
            operation: operation.into(),
            additional_data: AdditionalAttributeData::Full { slot, uuid },
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default, Serialize)]
#[serde(untagged)]
enum AdditionalAttributeData {
    #[default]
    None,
    Full {
        #[serde(rename = "Slot")]
        slot: EquipmentSlot,
        #[serde(rename = "UUID")]
        uuid: Uuid,
    },
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize)]
#[serde(into = "String")]
pub enum Attribute {
    // generic
    MaxHealth,
    FollowRange,
    KnockbackResistance,
    MovementSpeed,
    AttackDamage,
    Armor,
    ArmorToughness,
    AttackKnockback,
    AttackSpeed,
    Luck,

    // horse
    HorseJumpStrength,

    // flying (bees + parrots)
    FlyingSpeed,

    // zombies
    ZombieSpawnReinforcements,

    // a non-vanilla attribute
    Custom(String),
}

impl ToString for Attribute {
    fn to_string(&self) -> String {
        match self {
            Attribute::MaxHealth => "generic.max_health",
            Attribute::FollowRange => "generic.follow_range",
            Attribute::KnockbackResistance => "generic.knockback_resistance",
            Attribute::MovementSpeed => "generic.movement_speed",
            Attribute::AttackDamage => "generic.attack_damage",
            Attribute::Armor => "generic.armor",
            Attribute::ArmorToughness => "generic.armor_toughness",
            Attribute::AttackKnockback => "generic.attack_knockback",
            Attribute::AttackSpeed => "generic.attack_speed",
            Attribute::Luck => "generic.luck",
            Attribute::HorseJumpStrength => "horse.jump_strength",
            Attribute::FlyingSpeed => "generic.flying_speed",
            Attribute::ZombieSpawnReinforcements => "zombie.spawn_reinforcements",
            Attribute::Custom(id) => id,
        }
        .to_string()
    }
}

impl From<Attribute> for String {
    fn from(attr: Attribute) -> Self {
        attr.to_string()
    }
}

pub enum AttributeOperation {
    Add,
    MultiplyBase,
    Multiply,
}

impl From<AttributeOperation> for i32 {
    fn from(val: AttributeOperation) -> Self {
        match val {
            AttributeOperation::Add => 0,
            AttributeOperation::MultiplyBase => 1,
            AttributeOperation::Multiply => 2,
        }
    }
}
