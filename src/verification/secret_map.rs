// TODO: Document

use std::{collections::HashMap, sync::{ Arc, RwLock }};

use uuid::Uuid;

use super::Secret;

type ArcHashmap = Arc<HashMap<Uuid, Arc<Secret>>>;

#[derive(Clone, Debug)]
pub struct SecretMap {
    inner_map: Arc<RwLock<Option<ArcHashmap>>>,
}

impl SecretMap {
    pub fn uninit() -> Self {
        Self { inner_map: Arc::new(RwLock::new(None)) }
    }

    pub fn get_inner_map(&self) -> Option<ArcHashmap> {
        let inner_opt_map = self.inner_map.read().ok()?;
        if let Some(inner_map) = inner_opt_map.as_ref() {
            return Some(inner_map.clone());
        }
        drop(inner_opt_map);

        let new_inner = Arc::new(HashMap::new());
        self.set_map_direct(new_inner.clone());
        Some(new_inner)
    }

    pub fn get(&self, key: &Uuid) -> Option<Arc<Secret>> {
        self.get_inner_map()?.get(key).cloned()
    }

    pub fn set_map(&self, map: HashMap<Uuid, Secret>) -> bool {
        let arc_map: ArcHashmap = Arc::new(
            map
                .into_iter()
                .map(|(key, value)| (key, Arc::new(value)))
                .collect()
        );
        self.set_map_direct(arc_map)
    }
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
