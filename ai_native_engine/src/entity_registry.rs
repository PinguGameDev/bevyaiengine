use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct EntityRegistry {
    entities: HashMap<String, Entity>,
}

impl EntityRegistry {
    pub fn register(&mut self, name: String, entity: Entity) -> Result<(), String> {
        if self.entities.contains_key(&name) {
            return Err(format!("Entity '{}' already exists", name));
        }
        self.entities.insert(name, entity);
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Option<Entity> {
        self.entities.remove(name)
    }

    pub fn get(&self, name: &str) -> Option<Entity> {
        self.entities.get(name).copied()
    }

    pub fn list(&self) -> Vec<String> {
        self.entities.keys().cloned().collect()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.entities.contains_key(name)
    }
}
