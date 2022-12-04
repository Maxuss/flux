use serde::Serialize;
use uuid::Uuid;

use crate::text::StringOr;

#[derive(Default, Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct HeadMeta {
    skull_owner: SkullOwner,
}

impl HeadMeta {
    pub fn new_player(username: String) -> Self {
        Self {
            skull_owner: SkullOwner::String(username),
        }
    }

    pub fn new(owner: CompoundSkullOwner) -> Self {
        Self {
            skull_owner: SkullOwner::Compound(owner),
        }
    }

    pub fn owner(&self) -> &SkullOwner {
        &self.skull_owner
    }

    pub fn owner_mut(&mut self) -> &mut SkullOwner {
        &mut self.skull_owner
    }

    pub fn set_owner<O: Into<SkullOwner>>(&mut self, owner: O) {
        self.skull_owner = owner.into()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum SkullOwner {
    String(String),
    Compound(CompoundSkullOwner),
}

impl Default for SkullOwner {
    fn default() -> Self {
        Self::String("Steve".to_owned())
    }
}

impl From<CompoundSkullOwner> for SkullOwner {
    fn from(value: CompoundSkullOwner) -> Self {
        Self::Compound(value)
    }
}

impl From<String> for SkullOwner {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for SkullOwner {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct CompoundSkullOwner {
    id: Uuid,
    name: String,
    properties: SkullProperties,
}

impl CompoundSkullOwner {
    pub fn new<S: Into<String>>(skin: S) -> Self {
        let id = Uuid::new_v4();
        let skin = skin.into();
        Self {
            id,
            name: id.to_string(),
            properties: SkullProperties {
                textures: vec![TextureContainer {
                    value: StringOr::String(skin),
                }],
            },
        }
    }

    pub fn from_url<S: Into<String>>(url: S) -> Self {
        let id = Uuid::new_v4();
        let url = url.into();
        let url = if url.starts_with("http://textures.minecraft.net/texture") {
            url
        } else {
            format!("http://textures.minecraft.net/texture/{url}")
        };

        Self {
            id,
            name: id.to_string(),
            properties: SkullProperties {
                textures: vec![TextureContainer {
                    value: StringOr::Other(
                        HeadTexture {
                            profile_name: None,
                            timestamp: None,
                            textures: ActualTexture {
                                cape: None,
                                skin: SkinData {
                                    url,
                                    metadata: SkinMetadata {
                                        model: SkinModel::Classic,
                                    },
                                },
                            },
                        }
                        .into(),
                    ),
                }],
            },
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_uuid(&mut self, id: Uuid) {
        self.id = id
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct TextureContainer {
    value: StringOr<HeadTexture>,
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct SkullProperties {
    textures: Vec<TextureContainer>,
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct HeadTexture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    pub textures: ActualTexture,
}

impl ToString for HeadTexture {
    fn to_string(&self) -> String {
        base64::encode_config(serde_json::to_string(self).unwrap(), base64::STANDARD)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "UPPERCASE")]
pub struct ActualTexture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cape: Option<CapeData>,
    pub skin: SkinData,
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct CapeData {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct SkinData {
    pub url: String,
    pub metadata: SkinMetadata,
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct SkinMetadata {
    pub model: SkinModel,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum SkinModel {
    Classic,
    Slim,
}
