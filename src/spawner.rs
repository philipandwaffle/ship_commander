pub mod spawner {
    use std::collections::VecDeque;

    use bevy::prelude::{default, Color, Commands, Plugin, Quat, ResMut, Transform, Vec3};
    use bevy_prototype_lyon::{
        prelude::{DrawMode, FillMode, GeometryBuilder},
        shapes::{self, RegularPolygon, RegularPolygonFeature},
    };

    use crate::{movement::movement::Movable, propelled_object::propelled_object::Projectile};

    pub struct SpawnerPlugin;
    impl Plugin for SpawnerPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.insert_resource(SpawnList::default())
                .add_system(spawn_entities);
        }
    }

    pub struct SpawnList {
        pub projectiles: VecDeque<(Vec3, Quat, Projectile)>,
    }
    impl Default for SpawnList {
        fn default() -> Self {
            Self {
                projectiles: VecDeque::new(),
            }
        }
    }

    pub fn spawn_entities(mut commands: Commands, mut spawns: ResMut<SpawnList>) {
        if !spawns.projectiles.is_empty() {
            spawn_projectile(commands, &mut spawns.projectiles);
        }
    }

    fn spawn_projectile(
        mut commands: Commands,
        projectiles: &mut VecDeque<(Vec3, Quat, Projectile)>,
    ) {
        let shape = create_reg_poly(3, 20.);
        while !projectiles.is_empty() {
            let (t, r, p) = projectiles.pop_front().unwrap();

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
                .insert(p);
        }
    }

    fn create_reg_poly(sides: usize, radius: f32) -> RegularPolygon {
        return shapes::RegularPolygon {
            sides: sides,
            feature: RegularPolygonFeature::Radius(radius),
            ..shapes::RegularPolygon::default()
        };
    }
}
