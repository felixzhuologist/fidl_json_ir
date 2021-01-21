use {
    serde::{Deserialize, Deserializer, Serialize, Serializer},
    std::{
        hash::{Hash, Hasher},
        ops::{Deref, DerefMut},
    },
};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct FileId(pub u32);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Span {
    pub file_id: FileId,
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Default, Clone)]
pub struct Spanned<T> {
    pub inner: T,
    pub span: Option<Span>,
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: Eq> Eq for Spanned<T> {}

impl<T: Hash> Hash for Spanned<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> Spanned<T> {
    pub fn with_span(inner: T, span: Span) -> Self {
        Spanned { inner, span: Some(span) }
    }

    pub fn without_span(inner: T) -> Self {
        Spanned { inner, span: None }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn span(&self) -> &Option<Span> {
        &self.span
    }
}

impl<T: Serialize> Serialize for Spanned<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Spanned<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self { inner: T::deserialize(deserializer)?, span: None })
    }
}
