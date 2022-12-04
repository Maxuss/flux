use serde::Serialize;

use crate::{
    effect::EffectKind,
    id::Identifier,
    item::{FireworkExplosion, ItemStack},
    loc::Position,
    util::GeneralColor,
};

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct BucketMeta {
    bucket_variant_tag: Option<u32>,
    entity_tag: Option<()>, // TODO: add entity tag later
}

impl BucketMeta {
    pub fn new() -> Self {
        Self {
            bucket_variant_tag: None,
            entity_tag: None,
        }
    }

    pub fn new_tropical(
        variant: FishVariant,
        body_color: GeneralColor,
        pattern_color: GeneralColor,
    ) -> Self {
        let mut bytes = [0u8; 4];
        let var_bytes = variant.bytes();
        bytes[0] = var_bytes[0];
        bytes[1] = var_bytes[1];
        bytes[2] = body_color as u8;
        bytes[3] = pattern_color as u8;
        Self {
            bucket_variant_tag: Some(u32::from_le_bytes(bytes)),
            entity_tag: None,
        }
    }
}

impl Default for BucketMeta {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum FishVariant {
    Flopper = 0,
    Stripey,
    Glitter,
    Blockfish,
    Betty,
    Clayfish,

    Kob,
    Sunstreak,
    Snooper,
    Dasher,
    Brinely,
    Spotty,
}

impl FishVariant {
    pub fn bytes(&self) -> [u8; 2] {
        let ord = *self as u8;
        let modif = if ord > 5 { 5 } else { 0 };
        [(ord > 5) as u8, ord - modif]
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct BundleMeta {
    items: Vec<ItemStack>,
}

impl BundleMeta {
    pub fn new(item_count: Option<usize>) -> Self {
        Self {
            items: Vec::with_capacity(if let Some(count) = item_count {
                count
            } else {
                1
            }),
        }
    }

    pub fn new_full(items: Vec<ItemStack>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &Vec<ItemStack> {
        &self.items
    }

    pub fn add_item(&mut self, item: ItemStack) {
        self.items.push(item);
    }

    pub fn add_items<I: Iterator<Item = ItemStack>>(&mut self, items: I) {
        self.items.extend(items)
    }
}

#[derive(Default, Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct CompassMeta {
    lodestone_tracked: bool,
    lodestone_dimension: Option<Identifier>,
    lodestone_pos: Option<Position>,
}

impl CompassMeta {
    pub fn new(
        tracked: bool,
        lodestone_dimension: Option<Identifier>,
        lodestone_pos: Option<Position>,
    ) -> Self {
        Self {
            lodestone_tracked: tracked,
            lodestone_dimension,
            lodestone_pos,
        }
    }

    pub fn tracked(&self) -> bool {
        self.lodestone_tracked
    }

    pub fn dimension(&self) -> &Option<Identifier> {
        &self.lodestone_dimension
    }

    pub fn position(&self) -> Option<Position> {
        self.lodestone_pos
    }

    pub fn set_tracked(&mut self, tracked: bool) {
        self.lodestone_tracked = tracked
    }

    pub fn set_position(&mut self, pos: Position) {
        self.lodestone_pos = Some(pos)
    }

    pub fn set_dimension(&mut self, dim: Identifier) {
        self.lodestone_dimension = Some(dim)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrossbowMeta {
    charged_projectiles: Vec<ItemStack>,
    charged: bool,
}

impl CrossbowMeta {
    pub fn uncharged() -> Self {
        Self {
            charged_projectiles: Vec::with_capacity(3),
            charged: false,
        }
    }

    pub fn charged(projectiles: Vec<ItemStack>) -> Self {
        Self {
            charged_projectiles: projectiles,
            charged: true,
        }
    }

    pub fn charged_projectiles(&self) -> &Vec<ItemStack> {
        &self.charged_projectiles
    }

    pub fn is_charged(&self) -> bool {
        self.charged
    }

    pub fn set_projectiles(&mut self, projectiles: Vec<ItemStack>) {
        self.charged_projectiles = projectiles
    }

    pub fn set_charged(&mut self, charged: bool) {
        self.charged = charged
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FireworkMeta {
    explosions: Vec<FireworkExplosion>,
    flight: i8,
}

impl FireworkMeta {
    pub fn new(explosions: Vec<FireworkExplosion>, flight_duration: i8) -> Self {
        Self {
            explosions,
            flight: flight_duration,
        }
    }

    pub fn explosions(&self) -> &Vec<FireworkExplosion> {
        &self.explosions
    }

    pub fn duration(&self) -> i8 {
        self.flight
    }

    pub fn set_explosions(&mut self, explosions: Vec<FireworkExplosion>) {
        self.explosions = explosions
    }

    pub fn set_duration(&mut self, duration: i8) {
        self.flight = duration
    }
}

#[derive(Default, Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct MapMeta {
    map: i32,
    decorations: Vec<MapDecoration>,
}

impl MapMeta {
    pub fn new(id: i32) -> Self {
        Self {
            map: id,
            decorations: Vec::with_capacity(4),
        }
    }

    pub fn id(&self) -> i32 {
        self.map
    }

    pub fn decorations(&self) -> &Vec<MapDecoration> {
        &self.decorations
    }

    pub fn add_decoration(&mut self, deco: MapDecoration) {
        self.decorations.push(deco)
    }

    pub fn set_decorations(&mut self, decos: Vec<MapDecoration>) {
        self.decorations = decos
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct MapDecoration {
    id: String,
    #[serde(rename = "type")]
    pub kind: MapDecorationKind,
    pub x: f64,
    pub z: f64,
    pub rot: f64,
}

impl MapDecoration {
    pub fn new(kind: MapDecorationKind, x: f64, z: f64, rotation: f64) -> Self {
        let bytes: [u8; 16] = rand::random();
        Self {
            id: base64::encode_config(bytes, base64::URL_SAFE_NO_PAD),
            kind,
            x,
            z,
            rot: rotation,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u8)]
pub enum MapDecorationKind {
    WhiteMarker,
    GreenMarker,
    RedMarker,
    BlueMarker,

    WhiteCross,
    RedTriangle,

    LargeWhiteDot,
    SmallWhiteDot,

    WoodlandMansion,
    OceanMonument,

    BannerWhite,
    BannerGray,
    BannerDarkGray,
    BannerBlack,
    BannerBrown,
    BannerRed,
    BannerOrange,
    BannerYellow,
    BannerLime,
    BannerGreen,
    BannerDarkAqua,
    BannerAqua,
    BannerBlue,
    BannerLightPurple,
    BannerDarkPurple,
    BannerPink,

    RedCross,
}

impl Serialize for MapDecorationKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuspiciousStewMeta {
    effects: Vec<StewEffect>,
}

impl SuspiciousStewMeta {
    pub fn new() -> Self {
        Self { effects: vec![] }
    }

    pub fn new_full(effects: Vec<StewEffect>) -> Self {
        Self { effects }
    }

    pub fn effects(&self) -> &Vec<StewEffect> {
        &self.effects
    }

    pub fn add_effect(&mut self, effect: StewEffect) {
        self.effects.push(effect)
    }
}

impl Default for SuspiciousStewMeta {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct StewEffect {
    pub effect_id: EffectKind,
    pub effect_duration: i32,
}
