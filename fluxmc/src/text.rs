use lobsterchat::component::Component;
use serde::Serialize;

pub type StrComponent = SerdeStr<Component>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SerdeStr<T: ToString>(T);

impl<T: ToString> SerdeStr<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }

    pub fn value(&self) -> &T {
        &self.0
    }
}

impl<T> Serialize for SerdeStr<T>
where
    T: ToString,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<T: ToString> From<T> for SerdeStr<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
