pub mod entity_manager {
    use std::collections::VecDeque;

    use bevy::{
        ecs::component::{ComponentStorage, TableStorage},
        prelude::{
            default, Color, Commands, Component, Entity, Plugin, Quat, Query, Res, ResMut,
            Transform, Vec3,
        },
        time::Time,
    };
    use bevy_prototype_lyon::{
        prelude::{DrawMode, FillMode, GeometryBuilder},
        shapes::{self, RegularPolygon, RegularPolygonFeature},
    };

    use crate::{
        movement::movement::Movable, propelled_object::propelled_object::Projectile,
        replicator::replicator::Replicable,
    };

    pub struct DespawnerPlugin;
    impl Plugin for DespawnerPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_system(despawn);
        }
    }

    pub fn despawn(
        mut commands: Commands,
        time: Res<Time>,
        despawns: Query<(Entity, &DespawnTime)>,
    ) {
        for (e, d) in despawns.iter() {
            if time.seconds_since_startup() > d.d_time {
                commands.entity(e).despawn();
            }
        }
    }

    pub struct SpawnerPlugin;
    impl Plugin for SpawnerPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.insert_resource(SpawnList::default())
                .add_system(spawn_projectile)
                .add_system(spawn_replicable);
        }
    }

    pub struct SpawnList {
        pub projectiles: VecDeque<(Vec3, Quat, f64, Projectile)>,
        pub replicables: VecDeque<Box<dyn Replicable + Send + Sync>>,
    }
    impl Default for SpawnList {
        fn default() -> Self {
            Self {
                projectiles: VecDeque::new(),
                replicables: VecDeque::new(),
            }
        }
    }

    fn spawn_projectile(mut commands: Commands, mut spawns: ResMut<SpawnList>, time: Res<Time>) {
        let shape = create_reg_poly(3, 20.);
        while !spawns.projectiles.is_empty() {
            let (t, r, dt, p) = spawns.projectiles.pop_front().unwrap();
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Fill(FillMode::color(Color::WHITE)),
                    Transform {
                        translation: t,
                        rotation: r,
                        ..default()
                    },
                ))
                .insert(Movable::default())
                .insert(p)
                .insert(DespawnTime {
                    d_time: time.seconds_since_startup() + dt,
                });
        }
    }

    fn create_reg_poly(sides: usize, radius: f32) -> RegularPolygon {
        return shapes::RegularPolygon {
            sides: sides,
            feature: RegularPolygonFeature::Radius(radius),
            ..shapes::RegularPolygon::default()
        };
    }

    #[derive(Component)]
    pub struct DespawnTime {
        d_time: f64,
    }

    fn spawn_replicable(mut commands: Commands, mut spawns: ResMut<SpawnList>) {
        while !spawns.replicables.is_empty() {
            commands.spawn_bundle(spawns.replicables.pop_front().unwrap().replicate());
        }
    }
}
