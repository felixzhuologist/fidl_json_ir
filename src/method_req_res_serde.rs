use {
    super::{MethodReqRes, Parameter, Spanned},
    serde::{de, ser::SerializeStruct, Deserializer, Serializer},
    std::fmt,
};

fn serialize_req_res<S: Serializer>(
    has_key: &'static str,
    params_key: &'static str,
    size_key: &'static str,
    request: Option<&MethodReqRes>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let is_some = request.is_some();
    let num_fields = if is_some { 3 } else { 1 };
    let mut ser = serializer.serialize_struct("", num_fields)?;
    ser.serialize_field(has_key, &is_some)?;
    if let Some(req) = request {
        ser.serialize_field(params_key, &*req.parameters)?;
        ser.serialize_field(size_key, &req.size)?;
    }
    ser.end()
}

struct ReqResVisitor {
    has_key: &'static str,
    params_key: &'static str,
    size_key: &'static str,
}

impl<'de> de::Visitor<'de> for ReqResVisitor {
    type Value = Option<MethodReqRes>;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}, {}, and {}", self.has_key, self.params_key, self.size_key)
    }
    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut has: Option<bool> = None;
        let mut params: Option<Vec<Spanned<Parameter>>> = None;
        let mut size: Option<u32> = None;
        while let Some(key) = map.next_key()? {
            if key == self.has_key {
                if has.is_some() {
                    return Err(de::Error::duplicate_field(self.has_key));
                }
                has = Some(map.next_value()?);
            } else if key == self.params_key {
                if params.is_some() {
                    return Err(de::Error::duplicate_field(self.params_key));
                }
                params = Some(map.next_value()?);
            } else if key == self.size_key {
                if size.is_some() {
                    return Err(de::Error::duplicate_field(self.params_key));
                }
                size = Some(map.next_value()?);
            } else {
                return Err(de::Error::unknown_field(key, &[]));
            }
        }
        let has = has.ok_or_else(|| de::Error::missing_field(self.has_key))?;
        if !has {
            if params.is_some() || size.is_some() {
                return Err(de::Error::custom(format!(
                    "{} was `false` but `{}` or `{}` were provided",
                    self.has_key, self.params_key, self.size_key,
                )));
            }
            return Ok(None);
        }
        let params = params.ok_or_else(|| de::Error::missing_field(self.params_key))?;
        Ok(Some(MethodReqRes { parameters: params, size: size.into() }))
    }
}

fn deserialize_req_res<'de, D: Deserializer<'de>>(
    has_key: &'static str,
    params_key: &'static str,
    size_key: &'static str,
    deserializer: D,
) -> Result<Option<MethodReqRes>, D::Error> {
    deserializer.deserialize_map(ReqResVisitor { has_key, params_key, size_key })
}

macro_rules! req_res_mod {
    ($mod_name:ident, $has_key:literal, $params_key:literal, $size_key:literal) => {
        pub(super) mod $mod_name {
            use super::*;
            pub(crate) fn serialize<S: Serializer>(
                request: &Option<Spanned<MethodReqRes>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                serialize_req_res(
                    $has_key,
                    $params_key,
                    $size_key,
                    request.as_ref().map(|x| &**x),
                    serializer
                )
            }
            pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Option<Spanned<MethodReqRes>>, D::Error> {
                deserialize_req_res($has_key, $params_key, $size_key, deserializer)
                    .map(|opt| opt.map(Spanned::without_span))
            }
        }
    };
}

req_res_mod!(request_ser, "has_request", "maybe_request", "maybe_request_size");
req_res_mod!(response_ser, "has_response", "maybe_response", "maybe_response_size");
