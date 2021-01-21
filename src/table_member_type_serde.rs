use {
    super::{Constant, Spanned, TableMemberType, Type},
    serde::{de, Deserialize, Deserializer, Serialize, Serializer},
};

/// Type only used for serialization from `TableMemberType`.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct SerTableMemberType<'a> {
    pub reserved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<&'a Spanned<Type>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a Spanned<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_default_value: &'a Option<Spanned<Constant>>,
}

/// Type only used for deserialization into `TableMemberType`.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct DeTableMemberType {
    pub reserved: bool,
    pub r#type: Option<Spanned<Type>>,
    pub name: Option<Spanned<String>>,
    pub maybe_default_value: Option<Spanned<Constant>>,
}

impl Serialize for TableMemberType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TableMemberType::Reserved => SerTableMemberType {
                reserved: true,
                r#type: None,
                name: None,
                maybe_default_value: &None,
            },
            TableMemberType::Field { r#type, name, maybe_default_value } => SerTableMemberType {
                reserved: false,
                r#type: Some(r#type),
                name: Some(name),
                maybe_default_value,
            },
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TableMemberType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut de_type = <DeTableMemberType>::deserialize(deserializer)?;
        let reserved = de_type.reserved;
        let r#type = de_type.r#type.take();
        let name = de_type.name.take();
        let maybe_default_value = de_type.maybe_default_value.take();

        if reserved {
            if r#type.is_some() || name.is_some() || maybe_default_value.is_some() {
                return Err(de::Error::custom(
                    "`reserved` was `true` but `type`, `name`, or `maybe_default_value` \
                     were provided",
                ));
            }

            Ok(TableMemberType::Reserved)
        } else {
            let r#type = r#type.ok_or_else(|| de::Error::missing_field("type"))?;
            let name = name.ok_or_else(|| de::Error::missing_field("name"))?;

            Ok(TableMemberType::Field { r#type, name, maybe_default_value })
        }
    }
}
