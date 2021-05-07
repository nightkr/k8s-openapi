// Generated from definition io.k8s.api.autoscaling.v1.HorizontalPodAutoscalerSpec

/// specification of a horizontal pod autoscaler.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscalerSpec {
    /// upper limit for the number of pods that can be set by the autoscaler; cannot be smaller than MinReplicas.
    pub max_replicas: i32,

    /// lower limit for the number of pods that can be set by the autoscaler, default 1.
    pub min_replicas: Option<i32>,

    /// reference to scaled resource; horizontal pod autoscaler will learn the current resource consumption and will set the desired number of pods by using its Scale subresource.
    pub scale_target_ref: crate::api::autoscaling::v1::CrossVersionObjectReference,

    /// target average CPU utilization (represented as a percentage of requested CPU) over all the pods; if not specified the default autoscaling policy will be used.
    pub target_cpu_utilization_percentage: Option<i32>,
}

impl<'de> crate::serde::Deserialize<'de> for HorizontalPodAutoscalerSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max_replicas,
            Key_min_replicas,
            Key_scale_target_ref,
            Key_target_cpu_utilization_percentage,
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
                            "maxReplicas" => Field::Key_max_replicas,
                            "minReplicas" => Field::Key_min_replicas,
                            "scaleTargetRef" => Field::Key_scale_target_ref,
                            "targetCPUUtilizationPercentage" => Field::Key_target_cpu_utilization_percentage,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HorizontalPodAutoscalerSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HorizontalPodAutoscalerSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_max_replicas: Option<i32> = None;
                let mut value_min_replicas: Option<i32> = None;
                let mut value_scale_target_ref: Option<crate::api::autoscaling::v1::CrossVersionObjectReference> = None;
                let mut value_target_cpu_utilization_percentage: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max_replicas => value_max_replicas = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_min_replicas => value_min_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scale_target_ref => value_scale_target_ref = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_target_cpu_utilization_percentage => value_target_cpu_utilization_percentage = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HorizontalPodAutoscalerSpec {
                    max_replicas: value_max_replicas.ok_or_else(|| crate::serde::de::Error::missing_field("maxReplicas"))?,
                    min_replicas: value_min_replicas,
                    scale_target_ref: value_scale_target_ref.ok_or_else(|| crate::serde::de::Error::missing_field("scaleTargetRef"))?,
                    target_cpu_utilization_percentage: value_target_cpu_utilization_percentage,
                })
            }
        }

        deserializer.deserialize_struct(
            "HorizontalPodAutoscalerSpec",
            &[
                "maxReplicas",
                "minReplicas",
                "scaleTargetRef",
                "targetCPUUtilizationPercentage",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HorizontalPodAutoscalerSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HorizontalPodAutoscalerSpec",
            2 +
            self.min_replicas.as_ref().map_or(0, |_| 1) +
            self.target_cpu_utilization_percentage.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxReplicas", &self.max_replicas)?;
        if let Some(value) = &self.min_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minReplicas", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scaleTargetRef", &self.scale_target_ref)?;
        if let Some(value) = &self.target_cpu_utilization_percentage {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetCPUUtilizationPercentage", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
