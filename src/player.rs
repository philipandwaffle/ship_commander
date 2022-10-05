pub mod player {

    use bevy::prelude::{App, Component, Plugin, Query, Res, With};

    use crate::{
        input::input::ControlStatus,
        propelled_object::propelled_object::{RotationInput, Ship, TranslationInput},
    };

    #[derive(Component)]
    pub struct Player;
    pub fn handle_player_inputs(
        input: Res<ControlStatus>,
        mut players: Query<&mut Ship, With<Player>>,
    ) {
        for mut p in players.iter_mut() {
            if input.forward && input.backward {
                p.ti = TranslationInput::Nothing;
            } else if input.forward {
                p.ti = TranslationInput::Forward;
            } else if input.backward {
                p.ti = TranslationInput::Backward;
            } else {
                p.ti = TranslationInput::Nothing;
            }

            if input.rotate_left && input.rotate_right {
                p.ri = RotationInput::Nothing;
            } else if input.rotate_right {
                p.ri = RotationInput::Right;
            } else if input.rotate_left {
                p.ri = RotationInput::Left;
            } else {
                p.ri = RotationInput::Nothing;
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
