use std::f32::consts::PI;
use std::vec;

use bevy::math::vec2;
use bevy::{math::vec3, prelude::*};
use bevy_prototype_lyon::prelude::*;
use entity_manager::entity_manager::{DespawnerPlugin, SpawnerPlugin};
use player::player::{handle_player_inputs, Player};
use propelled_object::propelled_object::{
    Projectile, PropulsionConstraints, PropulsionPlugin, PropulsionValues, RotationInput, Ship,
    TranslationInput,
};
use replicator::replicator::Replicable;

mod movement;
use crate::movement::movement::*;

mod input;
use crate::input::input::*;

mod player;

mod propelled_object;

mod entity_manager;

mod replicator;

fn main() {
    sandbox();

    App::new()
        .insert_resource(Time::default())
        .insert_resource(WindowDescriptor {
            title: "Physics Engine".to_string(),
            mode: bevy::window::WindowMode::Windowed,
            width: 600.,
            height: 400.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(PropulsionPlugin)
        .add_plugin(SpawnerPlugin)
        .add_plugin(DespawnerPlugin)
        .add_startup_system(setup_system)
        .add_startup_system(spawn_entities)
        .add_system(movement)
        .add_system(handle_player_inputs)
        .run();
}

fn sandbox() {
    let comp_vec: Vec<Box<dyn Replicable>> = vec![];
}

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>) {
    // camera
    let mut cam = Camera2dBundle::default();
    cam.projection.scale = 10.;
    commands.spawn_bundle(cam);
}

fn spawn_entities(mut commands: Commands) {
    let circle = create_reg_poly(4, 50.);

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform {
                translation: vec3(0., 0., 0.),
                ..default()
            },
        ))
        .insert(Movable::default())
        .insert(Ship {
            pc: PropulsionConstraints {
                max_acc: 4.,
                max_vel: 30.,
                max_a_acc: 2. * PI / 8.,
                max_a_vel: 4. * PI / 8.,
            },
            pv: PropulsionValues {
                f_acc: 4.,
                b_acc: 10.,
                a_acc: PI / 8.,
            },
            ti: TranslationInput::Nothing,
            ri: RotationInput::Nothing,
            ammo: vec![Projectile {
                pc: PropulsionConstraints {
                    max_acc: 200000.,
                    max_vel: 1000.,
                    max_a_acc: 2. * PI / 8.,
                    max_a_vel: 4. * PI / 8.,
                },
                pv: PropulsionValues {
                    f_acc: 200000.,
                    b_acc: 0.,
                    a_acc: 0.,
                },
            }],
        })
        .insert(Player);
}

fn create_reg_poly(sides: usize, radius: f32) -> RegularPolygon {
    return shapes::RegularPolygon {
        sides: sides,
        feature: RegularPolygonFeature::Radius(radius),
        ..shapes::RegularPolygon::default()
    };
}
