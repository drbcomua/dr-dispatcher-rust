use crate::name::Name;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReactiveStorage {
    names: Arc<RwLock<DashMap<Name, ()>>>,
}

impl ReactiveStorage {
    pub fn new() -> Self {
        Self {
            names: Arc::new(RwLock::new(DashMap::new())),
        }
    }

    pub async fn dispatch(&self, input_names: Vec<Name>) -> Vec<Name> {
        let mut existing_names = Vec::new();
        let names_lock = self.names.write().await;

        for name in input_names {
            if names_lock.contains_key(&name) {
                existing_names.push(name);
            } else {
                names_lock.insert(name, ());
            }
        }

        existing_names
    }

    pub async fn remove(&self, input_names: Vec<Name>) {
        let names_lock = self.names.write().await;

        for name in input_names {
            names_lock.remove(&name);
        }
    }
}
