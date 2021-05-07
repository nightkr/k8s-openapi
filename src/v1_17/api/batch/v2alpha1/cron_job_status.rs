// Generated from definition io.k8s.api.batch.v2alpha1.CronJobStatus

/// CronJobStatus represents the current state of a cron job.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CronJobStatus {
    /// A list of pointers to currently running jobs.
    pub active: Option<Vec<crate::api::core::v1::ObjectReference>>,

    /// Information when was the last time the job was successfully scheduled.
    pub last_schedule_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl<'de> crate::serde::Deserialize<'de> for CronJobStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_active,
            Key_last_schedule_time,
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
                            "active" => Field::Key_active,
                            "lastScheduleTime" => Field::Key_last_schedule_time,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CronJobStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CronJobStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_active: Option<Vec<crate::api::core::v1::ObjectReference>> = None;
                let mut value_last_schedule_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active => value_active = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_schedule_time => value_last_schedule_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CronJobStatus {
                    active: value_active,
                    last_schedule_time: value_last_schedule_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "CronJobStatus",
            &[
                "active",
                "lastScheduleTime",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CronJobStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CronJobStatus",
            self.active.as_ref().map_or(0, |_| 1) +
            self.last_schedule_time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.active {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "active", value)?;
        }
        if let Some(value) = &self.last_schedule_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastScheduleTime", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
