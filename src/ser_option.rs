use {
    serde::{ser::Error, Deserialize, Deserializer, Serialize, Serializer},
    std::ops::{Deref, DerefMut},
};

/// Custom type wrapping `std::option::Option` used for fields that may not be resolved on creation
/// but must be present during attempts to `Serialize` and `Deserialize`.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct SerOption<T>(Option<T>);

impl<T> Default for SerOption<T> {
    fn default() -> Self {
        None.into()
    }
}

impl<T> Deref for SerOption<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Option<T> {
        &self.0
    }
}

impl<T> DerefMut for SerOption<T> {
    fn deref_mut(&mut self) -> &mut Option<T> {
        &mut self.0
    }
}

impl<T> From<Option<T>> for SerOption<T> {
    fn from(opt: Option<T>) -> Self {
        SerOption(opt)
    }
}

impl<T: Serialize> Serialize for SerOption<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0
            .as_ref()
            .ok_or_else(|| S::Error::custom("unexpected unresolved value"))?
            .serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for SerOption<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self(Some(T::deserialize(deserializer)?)))
    }
}
