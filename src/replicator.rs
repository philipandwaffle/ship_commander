pub mod replicator {
    use std::sync::Arc;

    use bevy::prelude::{Component, Query, ResMut, Transform, Vec2};
    use bevy_prototype_lyon::prelude::tess::math::Translation;

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
        master_copy: Arc<dyn Replicable + Sync + Send>,
        status: Status,
        translation_offset: Vec2,
    }
    impl Replicator {
        fn replicate_master(&self) {
            self.master_copy.replicate();
        }
    }
    fn process_replicators(sl: ResMut<SpawnList>, replicators: Query<(&Transform, &Replicator)>) {
        for (t, r) in replicators.iter() {
            
        }
    }
}
