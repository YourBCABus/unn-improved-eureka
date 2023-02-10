//! This module contains the 
// TODO: Document

use std::{collections::HashMap, sync::{ Arc, Mutex, MutexGuard }, ops::DerefMut, fmt::Debug};

use uuid::Uuid;

use super::Secret;

/// A simple shortening of a shared [`Arc`] type for constant usage.
type ArcHashmap = Arc<HashMap<Uuid, Arc<Secret>>>;

/// A struct for easy wrapping of a thread-global client ID to client secret map.
#[derive(Debug)]
pub struct SecretMap {
    /// The actual data within the SecretMap. It's a shared, interior mutable, optional, shared hashmap from Uuids to shared secrets.
    inner_map: Mutex<Option<ArcHashmap>>,
}

#[allow(dead_code)]
impl SecretMap {
    /// Create a struct that contains a SecretMap without initializing any data.
    pub const fn uninit() -> Self {
        Self { inner_map: Mutex::new(None) }
    }

    /// Returns either the contained or a newly-initialized ArcHashmap,
    /// only returning None if it failed to access or reset the underlying map.
    pub fn get_inner_map(&self) -> Option<ArcHashmap> {
        let inner_opt_map = self.get_unwrapped_guard_opt();
        if let Some(inner_map) = inner_opt_map.as_ref() {
            return Some(Arc::clone(inner_map));
        }

        let new_inner = Arc::new(HashMap::new());
        if self.set_map_direct(new_inner.clone()) {
            Some(new_inner)
        } else {
            None
        }
    }

    /// This gets a reference-counted clone of the HashMap containing the mapping.
    pub fn get(&self, key: &Uuid) -> Option<Arc<Secret>> {
        self.get_unwrapped_guard_opt()?.get(key).cloned()
    }

    // TODO: Actually hook this up to the DB.
    /// This resets the inner HashMap, with crappy thread-safety allowing for access and setting overlap.
    pub fn set_map(&self, map: HashMap<Uuid, Secret>) -> bool {
        let arc_map: ArcHashmap = Arc::new(
            map
                .into_iter()
                .map(|(key, value)| (key, Arc::new(value)))
                .collect()
        );
        self.set_map_direct(arc_map)
    }

    /// This function is used directly by the `set_map` and `get_inner_map` associated methods,
    /// This allows the inner map to be directly passed in rather than run through the [`ArcHashmap`] conversion. 
    fn set_map_direct(&self, arc_map: ArcHashmap) -> bool {

        if let Some(mut guard) = self.get_guard_opt() {
            *guard = Some(arc_map);
            true
        } else {
            false
        }
    }

    /// This function returns an [Option] of a custom guard for a ArcHashmap.
    /// The custom guard allows for the usage of the ArcHashmap directly, instead of the nested `if` statements.
    fn get_guard_opt(&self) -> Option<impl DerefMut<Target = Option<ArcHashmap>> + '_> {
        let lock = self.inner_map.try_lock().ok()?;
        Some(lock)
    }

    /// This function returns an [Option] of a custom guard for a ArcHashmap.
    /// The custom guard allows for the usage of the ArcHashmap directly, instead of the nested `if` statements.
    fn get_unwrapped_guard_opt(&self) -> Option<impl DerefMut<Target = ArcHashmap> + '_> {
        let lock = self.inner_map.try_lock().ok()?;
        SecretMapMutexGuardUnwrap::try_new(lock)
    }
}

/// A struct that acts as if it were a `MutexGuard<'a, ArcHashmap>`,
/// but which can be created from a `MutexGuard<'a, Option<ArcHashmap>>` (when the variant is `Some`).
pub struct SecretMapMutexGuardUnwrap<'a>(
    /// The inner mutex guard to protect
    MutexGuard<'a, Option<ArcHashmap>>
);

impl<'a> SecretMapMutexGuardUnwrap<'a> {
    /// Creates a SecretMapMutexGuardUnwrap that is guaranteed to never panic, despite calling `unwrap`.
    /// 
    /// If the guard contains a `None`, it will return `None`.
    /// If it contains a `Some`, it will return a `Some(Self)`.
    pub fn try_new(lock: MutexGuard<'a, Option<ArcHashmap>>) -> Option<Self> {
        if lock.is_some() {
            Some(Self(lock))
        } else {
            None
        }
    }
}
impl core::ops::Deref for SecretMapMutexGuardUnwrap<'_> {
    type Target = ArcHashmap;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}
impl core::ops::DerefMut for SecretMapMutexGuardUnwrap<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}

