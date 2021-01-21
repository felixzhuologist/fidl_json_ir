use {
    super::DeclPath,
    serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer},
};

impl Serialize for DeclPath {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        [&self.library_name, "/", &self.decl_name].concat().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DeclPath {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let str_path = <&'de str>::deserialize(deserializer)?;
        let mut elems = str_path.split('/');
        let library_name =
            elems.next().ok_or_else(|| D::Error::custom("empty declaration path"))?.into();
        let decl_name =
            elems.next().ok_or_else(|| D::Error::custom("no `/` in declaration path"))?.into();
        if elems.next().is_some() {
            return Err(D::Error::custom("multiple `/`s in declaration path"))?;
        }
        Ok(DeclPath { library_name, decl_name })
    }
}
