/// A trait applies to types that support deep merging.
///
/// `a.merge_from(b)` behaves in the following ways:
///
/// ## `struct`s
///
/// Structs are merged by individually merging each of their fields. For example, given:
///
/// ```rust,ignore
/// struct S {
///     a: i32,
///     b: String,
/// }
/// ```
///
/// ... the expected impl of `DeepMerge` for `S` would be:
///
/// ```rust,ignore
/// impl DeepMerge for S {
///     fn merge_from(&mut self, other: Self) {
///         self.a.merge_from(other.a);
///         self.b.merge_from(other.b);
///     }
/// }
/// ```
///
/// The structs in the `k8s-openapi` crate behave this way. If you are implementing this trait for your own types, it is recommended to impl it in the same way.
///
/// ## `Option`
///
/// - If `b` is a `None`, `a` is unchanged.
///
/// - If `b` is a `Some(b_inner)`:
///
///   - If `a` is a `Some(a_inner)`, `a_inner` is merged with `b_inner`.
///
///   - If `a` is a `None`, `a` becomes `Some(b_inner)`.
///
/// ## `Vec`
///
/// Use an [explicit merge strategy](`strategies::list`).
///
/// ## `BTreeMap`
///
/// Use an [explicit merge strategy](`strategies::map`).
///
/// ## `serde_json::Value`
///
/// `serde_json::Value` is merged using the JSON merge algorithm (RFC 7396).
///
/// ## Other types
///
/// `self` is just replaced by `other`.
pub trait DeepMerge {
    /// Merge `other` into `self`.
    fn merge_from(&mut self, other: Self);
}

macro_rules! default_overwriting_impl {
    () => {
        fn merge_from(&mut self, other: Self) {
            *self = other;
        }
    };
}

impl DeepMerge for bool { default_overwriting_impl! {} }
impl DeepMerge for i32 { default_overwriting_impl! {} }
impl DeepMerge for i64 { default_overwriting_impl! {} }
impl DeepMerge for f64 { default_overwriting_impl! {} }
impl DeepMerge for String { default_overwriting_impl! {} }
impl DeepMerge for crate::ByteString { default_overwriting_impl! {} }
impl<Tz> DeepMerge for chrono::DateTime<Tz> where Tz: chrono::TimeZone { default_overwriting_impl! {} }

impl DeepMerge for serde_json::Value {
    fn merge_from(&mut self, other: Self) {
        if let serde_json::Value::Object(this) = self {
            if let serde_json::Value::Object(other) = other {
                for (k, v) in other {
                    if v.is_null() {
                        this.remove(&k);
                    }
                    else {
                        this.entry(k).or_insert(serde_json::Value::Null).merge_from(v);
                    }
                }

                return;
            }
        }

        *self = other;
    }
}

impl<T> DeepMerge for Box<T> where T: DeepMerge {
    fn merge_from(&mut self, other: Self) {
        (**self).merge_from(*other);
    }
}

impl<T> DeepMerge for Option<T> where T: DeepMerge {
    fn merge_from(&mut self, other: Self) {
        if let Some(other) = other {
            if let Some(s) = self {
                s.merge_from(other);
            } else {
                *self = Some(other);
            }
        }
    }
}

/// Strategies for merging collections.
pub mod strategies {
    #[cfg(doc)]
    use std::collections::BTreeMap;

    /// Strategies for merging [`Vec`]s.
    ///
    /// These correspond to [`JSONSchemaProps.x-kubernetes-list-type`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.26/#jsonschemaprops-v1-apiextensions-k8s-io).
    pub mod list {
        use private::AsOptVec;

        mod private {
            pub trait AsOptVec {
                type Item;
                fn set(&mut self, new: Self);
                fn as_mut_opt(&mut self) -> Option<&mut Vec<Self::Item>>;
                fn into_opt(self) -> Option<Vec<Self::Item>>;
            }

            impl<T> AsOptVec for Vec<T> {
                type Item = T;
                fn set(&mut self, new: Self) {
                    *self = new;
                }
                fn as_mut_opt(&mut self) -> Option<&mut Self> {
                    Some(self)
                }
                fn into_opt(self) -> Option<Self> {
                    Some(self)
                }
            }
            impl<T> AsOptVec for Option<Vec<T>> {
                type Item = T;
                fn set(&mut self, new: Self) {
                    if new.is_some() {
                        *self = new;
                    }
                }
                fn as_mut_opt(&mut self) -> Option<&mut Vec<T>> {
                    self.as_mut()
                }
                fn into_opt(self) -> Self {
                    self
                }
            }
        }

        /// The whole list is treated as one scalar value, and will be replaced with the new (non-[`None`]) value.
        pub fn atomic<V>(old: &mut V, new: V) where V: AsOptVec {
            old.set(new);
        }
        /// The list is treated as a map.
        ///
        /// Any items with matching keys will be deep-merged. Any items that are found in `new` but not `old` will be appended to `old`.
        pub fn map<V>(
            old: &mut V,
            new: V,
            key_comparators: &[fn(&V::Item, &V::Item) -> bool],
            merge_item: fn(&mut V::Item, V::Item),
        )
        where
            V: AsOptVec,
        {
            if let Some(old) = old.as_mut_opt() {
                for new_item in new.into_opt().into_iter().flatten() {
                    if let Some(old_item) = old.iter_mut().find(|old_item| key_comparators.iter().all(|f| f(&new_item, old_item))) {
                        merge_item(old_item, new_item);
                    } else {
                        old.push(new_item);
                    }
                }
            } else {
                old.set(new);
            }
        }
        /// The list is treated as a set.
        ///
        /// Items from `new` will be appended to `old`, _unless_ `old` already contains an equal item.
        pub fn set<V>(old: &mut V, new: V) where V: AsOptVec, V::Item: PartialEq {
            if let Some(old) = old.as_mut_opt() {
                for item in new.into_opt().into_iter().flatten() {
                    if !old.contains(&item) {
                        old.push(item);
                    }
                }
            } else {
                old.set(new);
            }
        }
    }

    /// Strategies for merging [`BTreeMap`]s.
    ///
    /// These correspond to [`JSONSchemaProps.x-kubernetes-map-type`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.26/#jsonschemaprops-v1-apiextensions-k8s-io).
    pub mod map {
        use std::collections::btree_map::Entry;

        use private::AsOptMap;

        mod private {
            use std::collections::BTreeMap;

            pub trait AsOptMap {
                type Key;
                type Value;
                fn set(&mut self, new: Self);
                fn as_mut_opt(&mut self) -> Option<&mut BTreeMap<Self::Key, Self::Value>>;
                fn into_opt(self) -> Option<BTreeMap<Self::Key, Self::Value>>;
            }

            impl<K, V> AsOptMap for BTreeMap<K, V> {
                type Key = K;
                type Value = V;
                fn set(&mut self, new: Self) {
                    *self = new;
                }
                fn as_mut_opt(&mut self) -> Option<&mut Self> {
                    Some(self)
                }
                fn into_opt(self) -> Option<Self> {
                    Some(self)
                }
            }
            impl<K, V> AsOptMap for Option<BTreeMap<K, V>> {
                type Key = K;
                type Value = V;
                fn set(&mut self, new: Self) {
                    if new.is_some() {
                        *self = new;
                    }
                }
                fn as_mut_opt(&mut self) -> Option<&mut BTreeMap<K, V>> {
                    self.as_mut()
                }
                fn into_opt(self) -> Self {
                    self
                }
            }
        }


        /// Each value will be merged separately.
        pub fn granular<M>(old: &mut M, new: M, merge_value: fn(&mut M::Value, M::Value))
        where
            M: AsOptMap,
            M::Key: Ord,
        {
            if let Some(old) = old.as_mut_opt() {
                for (k, new_v) in new.into_opt().into_iter().flatten() {
                    match old.entry(k) {
                        Entry::Vacant(entry) => { entry.insert(new_v); }
                        Entry::Occupied(entry) => merge_value(entry.into_mut(), new_v),
                    }
                }
            } else {
                old.set(new);
            }
        }
        /// The whole map is treated as one scalar value, and will be replaced with the new (non-[`None`]) value.
        pub fn atomic<M>(old: &mut M, new: M) where M: AsOptMap {
            old.set(new);
        }
    }
}
