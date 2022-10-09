pub mod replicator {
    use std::sync::Arc;

    use bevy::prelude::{Component, Vec2};

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
        master_copy: Arc<dyn Replicable + Sync + Send>,
        status: Status,
        translation_offset: Vec2,
    }
    impl Replicator {
        fn replicate_master() {}
    }
}
