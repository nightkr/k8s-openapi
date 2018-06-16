// Generated from definition io.k8s.kubernetes.pkg.api.v1.Event

/// Event is a report of an event somewhere in the cluster.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Event {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// The number of times this event has occurred.
    pub count: Option<i32>,

    /// The time at which the event was first recorded. (Time of server receipt is in TypeMeta.)
    pub first_timestamp: Option<::v1_7::apimachinery::pkg::apis::meta::v1::Time>,

    /// The object that this event is about.
    pub involved_object: ::v1_7::kubernetes::pkg::api::v1::ObjectReference,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// The time at which the most recent occurrence of this event was recorded.
    pub last_timestamp: Option<::v1_7::apimachinery::pkg::apis::meta::v1::Time>,

    /// A human-readable description of the status of this operation.
    pub message: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: ::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// This should be a short, machine understandable string that gives the reason for the transition into the object's current status.
    pub reason: Option<String>,

    /// The component reporting this event. Should be a short machine understandable string.
    pub source: Option<::v1_7::kubernetes::pkg::api::v1::EventSource>,

    /// Type of this event (Normal, Warning), new types could be added in the future
    pub type_: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_count,
            Key_first_timestamp,
            Key_involved_object,
            Key_kind,
            Key_last_timestamp,
            Key_message,
            Key_metadata,
            Key_reason,
            Key_source,
            Key_type_,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "count" => Field::Key_count,
                            "firstTimestamp" => Field::Key_first_timestamp,
                            "involvedObject" => Field::Key_involved_object,
                            "kind" => Field::Key_kind,
                            "lastTimestamp" => Field::Key_last_timestamp,
                            "message" => Field::Key_message,
                            "metadata" => Field::Key_metadata,
                            "reason" => Field::Key_reason,
                            "source" => Field::Key_source,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Event;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Event")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_count: Option<i32> = None;
                let mut value_first_timestamp: Option<::v1_7::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_involved_object: Option<::v1_7::kubernetes::pkg::api::v1::ObjectReference> = None;
                let mut value_kind: Option<String> = None;
                let mut value_last_timestamp: Option<::v1_7::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_reason: Option<String> = None;
                let mut value_source: Option<::v1_7::kubernetes::pkg::api::v1::EventSource> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_count => value_count = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_first_timestamp => value_first_timestamp = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_involved_object => value_involved_object = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_timestamp => value_last_timestamp = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_reason => value_reason = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_source => value_source = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Event {
                    api_version: value_api_version,
                    count: value_count,
                    first_timestamp: value_first_timestamp,
                    involved_object: value_involved_object.ok_or_else(|| ::serde::de::Error::missing_field("involvedObject"))?,
                    kind: value_kind,
                    last_timestamp: value_last_timestamp,
                    message: value_message,
                    metadata: value_metadata.ok_or_else(|| ::serde::de::Error::missing_field("metadata"))?,
                    reason: value_reason,
                    source: value_source,
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "Event",
            &[
                "apiVersion",
                "count",
                "firstTimestamp",
                "involvedObject",
                "kind",
                "lastTimestamp",
                "message",
                "metadata",
                "reason",
                "source",
                "type",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Event",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.count.as_ref().map_or(0, |_| 1) +
            self.first_timestamp.as_ref().map_or(0, |_| 1) +
            1 +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.last_timestamp.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            1 +
            self.reason.as_ref().map_or(0, |_| 1) +
            self.source.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.count {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "count", value)?;
        }
        if let Some(value) = &self.first_timestamp {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "firstTimestamp", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "involvedObject", &self.involved_object)?;
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.last_timestamp {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "lastTimestamp", value)?;
        }
        if let Some(value) = &self.message {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.reason {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        if let Some(value) = &self.source {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "source", value)?;
        }
        if let Some(value) = &self.type_ {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}