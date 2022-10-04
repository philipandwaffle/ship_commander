use std::f32::consts::PI;

use bevy::math::vec2;
use bevy::{math::vec3, prelude::*};
use bevy_prototype_lyon::prelude::*;
use player::player::handle_player_inputs;

mod movement;
use crate::movement::movement::*;

mod input;
use crate::input::input::*;

mod player;

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
        .add_startup_system(setup_system)
        .add_startup_system(spawn_entities)
        .add_system(movement)
        .add_system(handle_player_inputs)
        .run();
}

fn sandbox() {
    let v = vec2(2., 2.);
    let up = vec2(0., 1.);
    let foo = Vec2::from_angle(PI / 4.);
    println!("{}", v.angle_between(up));
    println!("{}", foo);
}

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>) {
    // camera
    let mut cam = Camera2dBundle::default();
    cam.projection.scale = 2.;
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
        .insert(Movable {
            angular_vel: 0.,
            ..default()
        })
        .insert(MovementConstraints {
            max_acc: 100.,
            max_vel: 100.,
            max_a_acc: 2. * PI / 8.,
            max_a_vel: 4. * PI / 8.,
        });
}

fn create_reg_poly(sides: usize, radius: f32) -> RegularPolygon {
    return shapes::RegularPolygon {
        sides: sides,
        feature: RegularPolygonFeature::Radius(radius),
        ..shapes::RegularPolygon::default()
    };
}
