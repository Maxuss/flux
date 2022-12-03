use serde::Serialize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FireworkExplosion {
    colors: Vec<u32>,
    fade_colors: Option<Vec<u32>>,
    flicker: Option<bool>,
    trail: Option<bool>,
    _type: ExplosionKind,
}

impl FireworkExplosion {
    pub fn new(colors: Vec<u32>, kind: ExplosionKind) -> Self {
        Self {
            colors,
            fade_colors: None,
            flicker: None,
            trail: None,
            _type: kind,
        }
    }

    pub fn colors(&self) -> &Vec<u32> {
        &self.colors
    }

    pub fn fade_colors(&self) -> &Option<Vec<u32>> {
        &self.fade_colors
    }

    pub fn flicker(&self) -> bool {
        self.flicker.unwrap_or(false)
    }

    pub fn trail(&self) -> bool {
        self.trail.unwrap_or(false)
    }

    pub fn kind(&self) -> ExplosionKind {
        self._type
    }

    pub fn set_colors(&mut self, colors: Vec<u32>) {
        self.colors = colors
    }

    pub fn set_fade_colors(&mut self, colors: Vec<u32>) {
        self.fade_colors = Some(colors)
    }

    pub fn set_flicker(&mut self, toggle: bool) {
        self.flicker = Some(toggle)
    }

    pub fn set_trail(&mut self, toggle: bool) {
        self.trail = Some(toggle)
    }

    pub fn set_kind(&mut self, kind: ExplosionKind) {
        self._type = kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExplosionKind {
    SmallBall = 0,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

impl Serialize for ExplosionKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}
