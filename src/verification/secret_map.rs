// TODO: Document

use std::{collections::HashMap, sync::{ Arc, RwLock }};

use uuid::Uuid;

use super::Secret;

/// A simple shortening of a shared [`Arc`] type for constant usage.
type ArcHashmap = Arc<HashMap<Uuid, Arc<Secret>>>;

/// A struct for easy wrapping of a thread-global client ID to client secret map.
#[derive(Clone, Debug)]
pub struct SecretMap {
    /// The actual data within the SecretMap. It's a shared, interior mutable, optional, shared hashmap from Uuids to shared secrets.
    inner_map: Arc<RwLock<Option<ArcHashmap>>>,
}

impl SecretMap {
    /// Create a struct that contains a SecretMap without initializing any data.
    pub fn uninit() -> Self {
        Self { inner_map: Arc::new(RwLock::new(None)) }
    }

    /// Returns either the contained or a newly-initialized ArcHashmap,
    /// only returning None if it failed to access or reset the underlying map.
    pub fn get_inner_map(&self) -> Option<ArcHashmap> {
        let inner_opt_map = self.inner_map.read().ok()?;
        if let Some(inner_map) = inner_opt_map.as_ref() {
            return Some(inner_map.clone());
        }
        drop(inner_opt_map);

        let new_inner = Arc::new(HashMap::new());
        if self.set_map_direct(new_inner.clone()) {
            Some(new_inner)
        } else {
            None
        }
    }

    /// This gets a reference-counted clone of the HashMap containing the mapping.
    pub fn get(&self, key: &Uuid) -> Option<Arc<Secret>> {
        self.get_inner_map()?.get(key).cloned()
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
        let guard = self.inner_map.write();
        if let Ok(mut guard) = guard {
            *guard = Some(arc_map);
            true
        } else {
            false
        }
    }
}
