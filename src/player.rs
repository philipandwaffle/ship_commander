pub mod player {

    use bevy::{
        prelude::{App, Commands, Component, Plugin, Query, Res, ResMut, Transform, With},
        transform,
    };

    use crate::{
        input::input::ControlStatus,
        propelled_object::propelled_object::{RotationInput, Ship, TranslationInput},
        spawner::spawner::SpawnList,
    };

    #[derive(Component)]
    pub struct Player;
    pub fn handle_player_inputs(
        mut sl: ResMut<SpawnList>,
        mut input: ResMut<ControlStatus>,
        mut players: Query<(&mut Ship, &Transform), With<Player>>,
    ) {
        for (mut s, t) in players.iter_mut() {
            if input.forward && input.backward {
                s.ti = TranslationInput::Nothing;
            } else if input.forward {
                s.ti = TranslationInput::Forward;
            } else if input.backward {
                s.ti = TranslationInput::Backward;
            } else {
                s.ti = TranslationInput::Nothing;
            }

            if input.rotate_left && input.rotate_right {
                s.ri = RotationInput::Nothing;
            } else if input.rotate_right {
                s.ri = RotationInput::Right;
            } else if input.rotate_left {
                s.ri = RotationInput::Left;
            } else {
                s.ri = RotationInput::Nothing;
            }

            if input.shoot {
                let proj = s.ammo.get(0).unwrap().clone();
                sl.projectiles
                    .push_back((t.translation, t.rotation, proj.clone()));
                input.shoot = false;
            }
        }
    }

    struct PlayerPlugin;
    impl Plugin for PlayerPlugin {
        fn build(&self, app: &mut App) {
            app.add_system(handle_player_inputs);
        }
    }
}
