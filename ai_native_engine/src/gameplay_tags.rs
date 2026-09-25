use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct GameplayTagsPlugin;

impl Plugin for GameplayTagsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TagRegistry>();
    }
}

/// Central registry for all gameplay tags.
/// 
/// Tags are interned to u32 IDs for memory efficiency and fast comparisons.
/// Supports hierarchical relationships (parent/child) for automatic inheritance.
/// 
/// # Example
/// ```
/// // Register tag hierarchy
/// registry.register("Damage", None);
/// registry.register("Damage.Type", Some("Damage"));
/// registry.register("Damage.Type.Fire", Some("Damage.Type"));
/// 
/// // Query with automatic hierarchy
/// tags.has_parent_tag(&registry, "Damage"); // true if entity has Damage.Type.Fire
/// ```
#[derive(Resource)]
pub struct TagRegistry {
    next_id: u32,
    name_to_id: HashMap<String, u32>,
    id_to_name: HashMap<u32, String>,
    parent_map: HashMap<u32, u32>,
}

impl Default for TagRegistry {
    fn default() -> Self {
        Self {
            next_id: 0,
            name_to_id: HashMap::new(),
            id_to_name: HashMap::new(),
            parent_map: HashMap::new(),
        }
    }
}

impl TagRegistry {
    /// Register a new tag. Returns the tag's ID.
    /// 
    /// If the tag already exists, returns the existing ID.
    /// If a parent is specified, links this tag to its parent.
    pub fn register(&mut self, tag: &str, parent: Option<&str>) -> u32 {
        if let Some(&id) = self.name_to_id.get(tag) {
            return id;
        }
        
        let id = self.next_id;
        self.next_id += 1;
        
        self.name_to_id.insert(tag.to_string(), id);
        self.id_to_name.insert(id, tag.to_string());
        
        if let Some(parent_tag) = parent {
            if let Some(&parent_id) = self.name_to_id.get(parent_tag) {
                self.parent_map.insert(id, parent_id);
            }
        }
        
        id
    }
    
    /// Get the ID for a tag name.
    pub fn get_id(&self, tag: &str) -> Option<u32> {
        self.name_to_id.get(tag).copied()
    }
    
    /// Get the name for a tag ID.
    pub fn get_name(&self, id: u32) -> Option<&str> {
        self.id_to_name.get(&id).map(|s| s.as_str())
    }
    
    /// Get the parent ID of a tag.
    pub fn get_parent(&self, id: u32) -> Option<u32> {
        self.parent_map.get(&id).copied()
    }
    
    /// Check if a tag has a specific ancestor (recursive).
    pub fn has_ancestor(&self, child: u32, ancestor: u32) -> bool {
        let mut current = child;
        while let Some(parent) = self.parent_map.get(&current) {
            if *parent == ancestor {
                return true;
            }
            current = *parent;
        }
        false
    }
    
    /// Get all ancestors of a tag.
    pub fn get_ancestors(&self, id: u32) -> Vec<u32> {
        let mut ancestors = Vec::new();
        let mut current = id;
        while let Some(parent) = self.parent_map.get(&current) {
            ancestors.push(*parent);
            current = *parent;
        }
        ancestors
    }
    
    /// Get all registered tag names.
    pub fn all_tags(&self) -> impl Iterator<Item = &str> {
        self.id_to_name.values().map(|s| s.as_str())
    }
    
    /// Get the total number of registered tags.
    pub fn len(&self) -> usize {
        self.name_to_id.len()
    }
    
    /// Check if any tags are registered.
    pub fn is_empty(&self) -> bool {
        self.name_to_id.is_empty()
    }
}

/// Component that stores gameplay tags for an entity.
/// 
/// Tags are stored as u32 IDs for memory efficiency.
/// Use `TagRegistry` for name lookups and hierarchy queries.
/// 
/// # Example
/// ```rust
/// // Add tags to entity
/// let fire_id = registry.register("Damage.Type.Fire", Some("Damage.Type"));
/// tags.add(fire_id);
/// 
/// // Query tags
/// assert!(tags.has_tag(fire_id));
/// assert!(tags.has_parent_tag(&registry, registry.get_id("Damage.Type").unwrap()));
/// ```
#[derive(Component, Default, Clone, Debug)]
pub struct GameplayTags {
    tag_ids: HashSet<u32>,
}

impl GameplayTags {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a tag by ID.
    pub fn add(&mut self, tag_id: u32) {
        self.tag_ids.insert(tag_id);
    }
    
    /// Remove a tag by ID.
    pub fn remove(&mut self, tag_id: u32) {
        self.tag_ids.remove(&tag_id);
    }
    
    /// Check if entity has a specific tag.
    pub fn has_tag(&self, tag_id: u32) -> bool {
        self.tag_ids.contains(&tag_id)
    }
    
    /// Check if entity has any of the given tags.
    pub fn has_any_tag(&self, tag_ids: &[u32]) -> bool {
        tag_ids.iter().any(|id| self.has_tag(*id))
    }
    
    /// Check if entity has all of the given tags.
    pub fn has_all_tags(&self, tag_ids: &[u32]) -> bool {
        tag_ids.iter().all(|id| self.has_tag(*id))
    }
    
    /// Check if entity has a tag OR any tag that inherits from it.
    /// 
    /// This walks up the hierarchy for each owned tag.
    pub fn has_parent_tag(&self, registry: &TagRegistry, parent_id: u32) -> bool {
        for &tag_id in &self.tag_ids {
            if tag_id == parent_id || registry.has_ancestor(tag_id, parent_id) {
                return true;
            }
        }
        false
    }
    
    /// Check if entity has any tag matching the given parent (or the parent itself).
    pub fn has_any_parent_tag(&self, registry: &TagRegistry, parent_ids: &[u32]) -> bool {
        parent_ids.iter().any(|id| self.has_parent_tag(registry, *id))
    }
    
    /// Get all tag IDs owned by this entity.
    pub fn tag_ids(&self) -> &HashSet<u32> {
        &self.tag_ids
    }
    
    /// Get the number of tags.
    pub fn len(&self) -> usize {
        self.tag_ids.len()
    }
    
    /// Check if entity has any tags.
    pub fn is_empty(&self) -> bool {
        self.tag_ids.is_empty()
    }
    
    /// Get all tag names for this entity (for debugging).
    pub fn tag_names(&self, registry: &TagRegistry) -> Vec<String> {
        self.tag_ids
            .iter()
            .filter_map(|id| registry.get_name(*id).map(|s| s.to_string()))
            .collect()
    }
}

/// Helper to build gameplay tags with a fluent API.
pub struct GameplayTagsBuilder {
    tags: HashSet<u32>,
}

impl GameplayTagsBuilder {
    pub fn new() -> Self {
        Self { tags: HashSet::new() }
    }
    
    pub fn with_tag(mut self, tag_id: u32) -> Self {
        self.tags.insert(tag_id);
        self
    }
    
    pub fn build(self) -> GameplayTags {
        GameplayTags { tag_ids: self.tags }
    }
}

// Predefined tag constants for common use cases
pub mod tags {
    // Damage types
    pub const DAMAGE: &str = "Damage";
    pub const DAMAGE_TYPE: &str = "Damage.Type";
    pub const DAMAGE_TYPE_PHYSICAL: &str = "Damage.Type.Physical";
    pub const DAMAGE_TYPE_FIRE: &str = "Damage.Type.Fire";
    pub const DAMAGE_TYPE_ICE: &str = "Damage.Type.Ice";
    pub const DAMAGE_TYPE_LIGHTNING: &str = "Damage.Type.Lightning";
    pub const DAMAGE_TYPE_POISON: &str = "Damage.Type.Poison";
    
    // Status effects
    pub const STATUS: &str = "Status";
    pub const STATUS_STUNNED: &str = "Status.Stunned";
    pub const STATUS_BURNING: &str = "Status.Burning";
    pub const STATUS_FROZEN: &str = "Status.Frozen";
    pub const STATUS_POISONED: &str = "Status.Poisoned";
    pub const STATUS_SLOWED: &str = "Status.Slowed";
    
    // Factions
    pub const FACTION: &str = "Faction";
    pub const FACTION_PLAYER: &str = "Faction.Player";
    pub const FACTION_ENEMY: &str = "Faction.Enemy";
    pub const FACTION_NEUTRAL: &str = "Faction.Neutral";
    pub const FACTION_ALLY: &str = "Faction.Ally";
    
    // Abilities
    pub const ABILITY: &str = "Ability";
    pub const ABILITY_MELEE: &str = "Ability.Melee";
    pub const ABILITY_RANGED: &str = "Ability.Ranged";
    pub const ABILITY_MAGIC: &str = "Ability.Magic";
    
    // Resistances
    pub const RESISTANCE: &str = "Resistance";
    pub const RESISTANCE_FIRE: &str = "Resistance.Fire";
    pub const RESISTANCE_ICE: &str = "Resistance.Ice";
    pub const RESISTANCE_PHYSICAL: &str = "Resistance.Physical";
}

/// System to register common tags at startup.
pub fn register_common_tags(mut registry: ResMut<TagRegistry>) {
    use tags::*;
    
    // Damage hierarchy
    registry.register(DAMAGE, None);
    registry.register(DAMAGE_TYPE, Some(DAMAGE));
    registry.register(DAMAGE_TYPE_PHYSICAL, Some(DAMAGE_TYPE));
    registry.register(DAMAGE_TYPE_FIRE, Some(DAMAGE_TYPE));
    registry.register(DAMAGE_TYPE_ICE, Some(DAMAGE_TYPE));
    registry.register(DAMAGE_TYPE_LIGHTNING, Some(DAMAGE_TYPE));
    registry.register(DAMAGE_TYPE_POISON, Some(DAMAGE_TYPE));
    
    // Status effects
    registry.register(STATUS, None);
    registry.register(STATUS_STUNNED, Some(STATUS));
    registry.register(STATUS_BURNING, Some(STATUS));
    registry.register(STATUS_FROZEN, Some(STATUS));
    registry.register(STATUS_POISONED, Some(STATUS));
    registry.register(STATUS_SLOWED, Some(STATUS));
    
    // Factions
    registry.register(FACTION, None);
    registry.register(FACTION_PLAYER, Some(FACTION));
    registry.register(FACTION_ENEMY, Some(FACTION));
    registry.register(FACTION_NEUTRAL, Some(FACTION));
    registry.register(FACTION_ALLY, Some(FACTION));
    
    // Abilities
    registry.register(ABILITY, None);
    registry.register(ABILITY_MELEE, Some(ABILITY));
    registry.register(ABILITY_RANGED, Some(ABILITY));
    registry.register(ABILITY_MAGIC, Some(ABILITY));
    
    // Resistances
    registry.register(RESISTANCE, None);
    registry.register(RESISTANCE_FIRE, Some(RESISTANCE));
    registry.register(RESISTANCE_ICE, Some(RESISTANCE));
    registry.register(RESISTANCE_PHYSICAL, Some(RESISTANCE));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_register_and_get_id() {
        let mut registry = TagRegistry::default();
        let id = registry.register("Damage.Type.Fire", Some("Damage.Type"));
        assert_eq!(registry.get_id("Damage.Type.Fire"), Some(id));
        assert_eq!(registry.get_name(id), Some("Damage.Type.Fire"));
    }
    
    #[test]
    fn test_duplicate_registration() {
        let mut registry = TagRegistry::default();
        let id1 = registry.register("Damage.Type.Fire", None);
        let id2 = registry.register("Damage.Type.Fire", None);
        assert_eq!(id1, id2);
    }
    
    #[test]
    fn test_parent_hierarchy() {
        let mut registry = TagRegistry::default();
        let parent = registry.register("Damage.Type", None);
        let child = registry.register("Damage.Type.Fire", Some("Damage.Type"));
        
        assert!(registry.has_ancestor(child, parent));
        assert_eq!(registry.get_parent(child), Some(parent));
    }
    
    #[test]
    fn test_grandparent_hierarchy() {
        let mut registry = TagRegistry::default();
        let grandparent = registry.register("Damage", None);
        let parent = registry.register("Damage.Type", Some("Damage"));
        let child = registry.register("Damage.Type.Fire", Some("Damage.Type"));
        
        assert!(registry.has_ancestor(child, parent));
        assert!(registry.has_ancestor(child, grandparent));
        assert!(!registry.has_ancestor(parent, child));
    }
    
    #[test]
    fn test_gameplay_tags_add_remove() {
        let mut registry = TagRegistry::default();
        let id = registry.register("Damage.Type.Fire", None);
        
        let mut tags = GameplayTags::new();
        tags.add(id);
        assert!(tags.has_tag(id));
        
        tags.remove(id);
        assert!(!tags.has_tag(id));
    }
    
    #[test]
    fn test_has_any_and_has_all() {
        let mut registry = TagRegistry::default();
        let id1 = registry.register("Damage.Type.Fire", None);
        let id2 = registry.register("Status.Burning", None);
        
        let mut tags = GameplayTags::new();
        tags.add(id1);
        
        assert!(tags.has_any_tag(&[id1, id2]));
        assert!(!tags.has_all_tags(&[id1, id2]));
        assert!(tags.has_all_tags(&[id1]));
    }
    
    #[test]
    fn test_has_parent_tag() {
        let mut registry = TagRegistry::default();
        let parent = registry.register("Damage.Type", None);
        let child = registry.register("Damage.Type.Fire", Some("Damage.Type"));
        
        let mut tags = GameplayTags::new();
        tags.add(child);
        
        assert!(tags.has_tag(child));
        assert!(!tags.has_tag(parent));
        assert!(tags.has_parent_tag(&registry, parent));
    }
    
    #[test]
    fn test_tag_names() {
        let mut registry = TagRegistry::default();
        let id1 = registry.register("Damage.Type.Fire", None);
        let id2 = registry.register("Status.Burning", None);
        
        let mut tags = GameplayTags::new();
        tags.add(id1);
        tags.add(id2);
        
        let names = tags.tag_names(&registry);
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"Damage.Type.Fire".to_string()));
        assert!(names.contains(&"Status.Burning".to_string()));
    }
}
