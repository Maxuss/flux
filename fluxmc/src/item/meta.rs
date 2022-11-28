use serde::Serialize;

use crate::{attr::AttributeModifier, id::Identifier};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemMetaTag {
    damage: i32,
    unbreakable: bool,
    can_destroy: Vec<Identifier>,
    custom_model_data: i32,
    attribute_modifiers: Vec<AttributeModifier>,
    #[serde(rename = "display")]
    display: ItemDisplay,
    #[serde(flatten)]
    _container: ItemMeta,
}

#[derive(Debug, Clone, Serialize)]
pub struct ItemDisplay {
    leather_color: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(untagged)]
pub enum ItemMeta {
    #[default]
    EmptyMeta,
}
