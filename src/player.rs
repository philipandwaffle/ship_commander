pub mod player {
    use std::f32::consts::PI;

    use crate::{
        input::input::{ControlStatus, PlayerConstants},
        movement::movement::Movable,
    };
    use bevy::{math::vec2, prelude::*};
    use bevy_prototype_lyon::plugin;

    struct Player;
    pub fn handle_player_inputs(
        input: Res<ControlStatus>,
        c: Res<PlayerConstants>,
        mut players: Query<(&mut Transform, &mut Movable)>,
    ) {
        for (mut t, mut m) in players.iter_mut() {
            if input.forward {
                println!("hello");
                let acc_to_add = t.rotation.xyz().truncate();

                //let foo = vec2(1, 2).
                //m.acc += player.
                // t.rotation.
                // m.acc = minnumf32(1., 2.)
                t.rotation = t.rotation * Quat::from_rotation_z(PI / 4.);
            }
            if input.rotate_left {
                m.angular_acc += c.max_angular_acc.min(m.angular_acc + c.angular_acc);

                m.angular_vel = m.angular_vel.clamp(-c.max_angular_vel, c.max_angular_vel);
                // if currently rotating left
                //m.angular_vel = c.max_angular_vel.min(m.angular_vel);

                // // if currently rotating left
                // m.angular_vel = if m.angular_vel > 0. {
                //     // the angular velocity must become no bigger than -max_angular_vel
                //     -(c.max_angular_vel).max(-m.angular_vel)
                // } else {
                //     // the angular velocity must become no bigger than max_angular_vel
                //     (c.max_angular_vel).min(m.angular_vel)
                // }
            }
            if input.rotate_right {
                m.angular_acc -= c.max_angular_acc.min(m.angular_acc + c.angular_acc);

                m.angular_vel = m.angular_vel.clamp(-c.max_angular_vel, c.max_angular_vel);
                // // if currently rotating right
                // m.angular_vel = if m.angular_vel < 0. {
                //     // the angular velocity must become no bigger than -max_angular_vel
                //     -(c.max_angular_vel).max(-m.angular_vel)
                // } else {
                //     // the angular velocity must become no smaller than -max_angular_vel
                //     (c.max_angular_vel).min(m.angular_vel)
                // }
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
