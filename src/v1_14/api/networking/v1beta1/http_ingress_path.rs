// Generated from definition io.k8s.api.networking.v1beta1.HTTPIngressPath

/// HTTPIngressPath associates a path regex with a backend. Incoming urls matching the path are forwarded to the backend.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HTTPIngressPath {
    /// Backend defines the referenced service endpoint to which the traffic will be forwarded to.
    pub backend: crate::api::networking::v1beta1::IngressBackend,

    /// Path is an extended POSIX regex as defined by IEEE Std 1003.1, (i.e this follows the egrep/unix syntax, not the perl syntax) matched against the path of an incoming request. Currently it can contain characters disallowed from the conventional "path" part of a URL as defined by RFC 3986. Paths must begin with a '/'. If unspecified, the path defaults to a catch all sending traffic to the backend.
    pub path: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for HTTPIngressPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_backend,
            Key_path,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "backend" => Field::Key_backend,
                            "path" => Field::Key_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HTTPIngressPath;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HTTPIngressPath")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_backend: Option<crate::api::networking::v1beta1::IngressBackend> = None;
                let mut value_path: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_backend => value_backend = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HTTPIngressPath {
                    backend: value_backend.ok_or_else(|| crate::serde::de::Error::missing_field("backend"))?,
                    path: value_path,
                })
            }
        }

        deserializer.deserialize_struct(
            "HTTPIngressPath",
            &[
                "backend",
                "path",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HTTPIngressPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HTTPIngressPath",
            1 +
            self.path.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "backend", &self.backend)?;
        if let Some(value) = &self.path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
