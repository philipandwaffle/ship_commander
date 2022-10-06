pub mod replicator {
    use bevy::prelude::{Component, Vec2};

    use crate::entity_manager::entity_manager::SpawnList;

    pub trait Replicable {
        fn replicate(&self);
    }
    enum Status {
        Idle,
        Charging(f32),
        Charged,
        Cooling(f32),
    }
    #[derive(Component)]
    pub struct Replicator {
        master_copy: Box<dyn Replicable>,
        status: Status,
        translation_offset: Vec2,
    }
    impl Replicator {
        fn replicate_master() {}
    }
}
