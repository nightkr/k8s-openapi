// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.ManagedFieldsEntry

/// ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ManagedFieldsEntry {
    /// APIVersion defines the version of this resource that this field set applies to. The format is "group/version" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted.
    pub api_version: Option<String>,

    /// FieldsType is the discriminator for the different fields format and version. There is currently only one possible value: "FieldsV1"
    pub fields_type: Option<String>,

    /// FieldsV1 holds the first JSON version format as described in the "FieldsV1" type.
    pub fields_v1: Option<crate::apimachinery::pkg::apis::meta::v1::FieldsV1>,

    /// Manager is an identifier of the workflow managing these fields.
    pub manager: Option<String>,

    /// Operation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'.
    pub operation: Option<String>,

    /// Time is timestamp of when these fields were set. It should always be empty if Operation is 'Apply'
    pub time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl<'de> crate::serde::Deserialize<'de> for ManagedFieldsEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_fields_type,
            Key_fields_v1,
            Key_manager,
            Key_operation,
            Key_time,
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
                            "apiVersion" => Field::Key_api_version,
                            "fieldsType" => Field::Key_fields_type,
                            "fieldsV1" => Field::Key_fields_v1,
                            "manager" => Field::Key_manager,
                            "operation" => Field::Key_operation,
                            "time" => Field::Key_time,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ManagedFieldsEntry;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ManagedFieldsEntry")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_fields_type: Option<String> = None;
                let mut value_fields_v1: Option<crate::apimachinery::pkg::apis::meta::v1::FieldsV1> = None;
                let mut value_manager: Option<String> = None;
                let mut value_operation: Option<String> = None;
                let mut value_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fields_type => value_fields_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fields_v1 => value_fields_v1 = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_manager => value_manager = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operation => value_operation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_time => value_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ManagedFieldsEntry {
                    api_version: value_api_version,
                    fields_type: value_fields_type,
                    fields_v1: value_fields_v1,
                    manager: value_manager,
                    operation: value_operation,
                    time: value_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "ManagedFieldsEntry",
            &[
                "apiVersion",
                "fieldsType",
                "fieldsV1",
                "manager",
                "operation",
                "time",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ManagedFieldsEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ManagedFieldsEntry",
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.fields_type.as_ref().map_or(0, |_| 1) +
            self.fields_v1.as_ref().map_or(0, |_| 1) +
            self.manager.as_ref().map_or(0, |_| 1) +
            self.operation.as_ref().map_or(0, |_| 1) +
            self.time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.fields_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fieldsType", value)?;
        }
        if let Some(value) = &self.fields_v1 {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fieldsV1", value)?;
        }
        if let Some(value) = &self.manager {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "manager", value)?;
        }
        if let Some(value) = &self.operation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "operation", value)?;
        }
        if let Some(value) = &self.time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "time", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ManagedFieldsEntry {
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.ManagedFieldsEntry".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: IntoIterator::into_iter([
                    (
                        "apiVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("APIVersion defines the version of this resource that this field set applies to. The format is \"group/version\" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fieldsType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FieldsType is the discriminator for the different fields format and version. There is currently only one possible value: \"FieldsV1\"".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fieldsV1".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::FieldsV1>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FieldsV1 holds the first JSON version format as described in the \"FieldsV1\" type.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "manager".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Manager is an identifier of the workflow managing these fields.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "operation".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Operation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "time".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Time is timestamp of when these fields were set. It should always be empty if Operation is 'Apply'".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ]).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
