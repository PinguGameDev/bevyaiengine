use bevy::prelude::*;

/// Health component for any entity that can take damage.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub is_dead: bool,
}

impl Health {
    /// Creates a new `Health` component with the given maximum value.
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            is_dead: false,
        }
    }

    /// Returns the current health as a ratio of max health (0.0..=1.0).
    pub fn ratio(&self) -> f32 {
        if self.max <= 0.0 {
            return 0.0;
        }
        (self.current / self.max).clamp(0.0, 1.0)
    }

    /// Restores health, capped at `max`.
    pub fn heal(&mut self, amount: f32) {
        if self.is_dead || amount <= 0.0 {
            return;
        }
        self.current = (self.current + amount).min(self.max);
    }

    /// Applies raw damage without triggering events. Returns true if lethal.
    pub fn take_damage(&mut self, amount: f32) -> bool {
        if self.is_dead || amount <= 0.0 {
            return false;
        }
        self.current = (self.current - amount).max(0.0);
        if self.current <= 0.0 {
            self.is_dead = true;
            true
        } else {
            false
        }
    }
}

/// Event triggered when an entity should take damage.
#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
    /// Optional GameplayTags damage type tag id (e.g., Damage.Type.Fire).
    pub damage_type: Option<u32>,
}

impl DamageEvent {
    pub fn new(target: Entity, amount: f32) -> Self {
        Self {
            target,
            amount,
            damage_type: None,
        }
    }

    pub fn with_damage_type(mut self, damage_type: u32) -> Self {
        self.damage_type = Some(damage_type);
        self
    }
}

/// Event triggered when an entity dies.
#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct DeathEvent {
    pub entity: Entity,
}

/// Plugin that wires up the health and damage observer system.
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_damage)
            .add_observer(on_death);
    }
}

/// Observer that applies damage to the target entity and triggers a death event if lethal.
fn on_damage(
    trigger: On<DamageEvent>,
    mut health_query: Query<&mut Health>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let Ok(mut health) = health_query.get_mut(event.target) else {
        return;
    };

    if health.take_damage(event.amount) {
        commands.trigger(DeathEvent {
            entity: event.target,
        });
    }
}

/// Observer that handles entity death by marking it as dead.
///
/// Currently marks the entity as dead without despawning, which keeps registry entries
/// valid and allows resurrection/game-over logic later.
fn on_death(trigger: On<DeathEvent>, mut health_query: Query<&mut Health>) {
    let event = trigger.event();
    if let Ok(mut health) = health_query.get_mut(event.entity) {
        health.is_dead = true;
        health.current = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_new() {
        let health = Health::new(100.0);
        assert_eq!(health.current, 100.0);
        assert_eq!(health.max, 100.0);
        assert!(!health.is_dead);
        assert_eq!(health.ratio(), 1.0);
    }

    #[test]
    fn test_take_damage() {
        let mut health = Health::new(100.0);
        assert!(!health.take_damage(30.0));
        assert_eq!(health.current, 70.0);
        assert!(!health.is_dead);
        assert!(health.take_damage(80.0));
        assert_eq!(health.current, 0.0);
        assert!(health.is_dead);
    }

    #[test]
    fn test_take_damage_when_dead() {
        let mut health = Health::new(100.0);
        health.take_damage(100.0);
        assert!(!health.take_damage(10.0));
        assert_eq!(health.current, 0.0);
    }

    #[test]
    fn test_heal() {
        let mut health = Health::new(100.0);
        health.take_damage(50.0);
        health.heal(20.0);
        assert_eq!(health.current, 70.0);
        health.heal(100.0);
        assert_eq!(health.current, 100.0);
    }

    #[test]
    fn test_heal_when_dead() {
        let mut health = Health::new(100.0);
        health.take_damage(100.0);
        health.heal(50.0);
        assert_eq!(health.current, 0.0);
        assert!(health.is_dead);
    }

    #[test]
    fn test_ratio_clamps() {
        let mut health = Health::new(100.0);
        health.take_damage(150.0);
        assert_eq!(health.ratio(), 0.0);
    }

    #[test]
    fn test_damage_observer_applies_damage() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, CombatPlugin));

        let entity = app.world_mut().spawn(Health::new(100.0)).id();
        app.world_mut()
            .trigger(DamageEvent::new(entity, 60.0));

        let health = app.world().entity(entity).get::<Health>().unwrap();
        assert_eq!(health.current, 40.0);
        assert!(!health.is_dead);
    }

    #[test]
    fn test_damage_observer_kills_entity() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, CombatPlugin));

        let entity = app.world_mut().spawn(Health::new(100.0)).id();
        app.world_mut()
            .trigger(DamageEvent::new(entity, 150.0));

        let health = app.world().entity(entity).get::<Health>().unwrap();
        assert!(health.is_dead);
        assert_eq!(health.current, 0.0);
    }

    #[test]
    fn test_damage_observer_ignores_dead_entities() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, CombatPlugin));

        let entity = app.world_mut().spawn(Health::new(100.0)).id();
        app.world_mut()
            .trigger(DamageEvent::new(entity, 100.0));
        app.world_mut()
            .trigger(DamageEvent::new(entity, 50.0));

        let health = app.world().entity(entity).get::<Health>().unwrap();
        assert_eq!(health.current, 0.0);
    }

    #[test]
    fn test_damage_with_type() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, CombatPlugin));

        let entity = app.world_mut().spawn(Health::new(100.0)).id();
        app.world_mut().trigger(
            DamageEvent::new(entity, 25.0).with_damage_type(42),
        );

        let health = app.world().entity(entity).get::<Health>().unwrap();
        assert_eq!(health.current, 75.0);
    }
}
