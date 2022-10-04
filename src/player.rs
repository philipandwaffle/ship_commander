pub mod player {
    use std::f32::consts::PI;

    use crate::{
        input::input::{ControlStatus, PlayerConstants},
        movement::movement::{Movable, MovementConstraints},
    };
    use bevy::{math::vec2, prelude::*};
    use bevy_prototype_lyon::plugin;

    #[derive(Component)]
    struct Player;
    pub fn handle_player_inputs(
        input: Res<ControlStatus>,
        pc: Res<PlayerConstants>,
        mut players: Query<(&mut Transform, &mut Movable, &MovementConstraints)>,
    ) {
        for (mut t, mut m, mc) in players.iter_mut() {
            if input.forward {
                println!("{}", (t.rotation.xyz() * pc.forward_acc).truncate());
                println!("{}", t.rotation.to_scaled_axis());
                m.add_acc((t.rotation.xyz() * pc.forward_acc).truncate(), &Some(mc));
            }
            if input.rotate_left {
                m.add_a_acc(pc.angular_acc, &Some(mc));
            }
            if input.rotate_right {
                m.add_a_acc(-pc.angular_acc, &Some(mc));
            }
        }
    }

    struct PlayerPlugin;
    impl Plugin for PlayerPlugin {
        fn build(&self, app: &mut App) {
            todo!()
        }
    }
}
