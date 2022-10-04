pub mod movement {
    use std::{
        f32::consts::PI,
        ops::{Add, Mul},
    };

    use bevy::prelude::*;
    #[derive(Component)]
    pub struct Movable {
        pub acc: Vec2,
        pub vel: Vec2,
        pub angular_acc: f32,
        pub angular_vel: f32,
    }
    impl Default for Movable {
        fn default() -> Self {
            Self {
                acc: default(),
                vel: default(),
                angular_acc: default(),
                angular_vel: default(),
            }
        }
    }
    impl Movable {
        pub fn apply_acc(&mut self, delta_time: &f32) {
            self.vel += self.acc * *delta_time;
            self.angular_vel += self.angular_acc * *delta_time;
        }

        pub fn apply_constrained_acc(&mut self, delta_time: &f32, mc: &MovementConstraints) {
            self.vel += self.acc * *delta_time;
            if self.vel.length() > mc.max_vel {
                self.vel = self.vel.normalize() * mc.max_vel;
            }
            self.angular_vel += self.angular_acc * *delta_time;
            self.angular_vel = self.angular_vel.clamp(-mc.max_a_vel, mc.max_a_vel);
        }

        pub fn add_acc(&mut self, acc: Vec2, mc: &Option<&MovementConstraints>) {
            self.acc += acc;
            match mc {
                Some(s) => {
                    self.acc += acc;
                    if self.acc.length() > s.max_acc {
                        self.acc = self.acc.normalize() * s.max_a_acc;
                    }
                }
                None => {}
            }
        }

        pub fn add_a_acc(&mut self, acc: f32, mc: &Option<&MovementConstraints>) {
            self.angular_acc += acc;
            match mc {
                Some(s) => self.angular_acc = self.angular_acc.clamp(-s.max_a_acc, s.max_a_acc),
                None => {}
            }
        }
    }

    #[derive(Component)]
    pub struct MovementConstraints {
        pub max_acc: f32,
        pub max_vel: f32,
        pub max_a_acc: f32,
        pub max_a_vel: f32,
    }

    pub fn movement(
        time: Res<Time>,
        mut movables: Query<(&mut Transform, &mut Movable, Option<&MovementConstraints>)>,
    ) {
        for (mut t, mut m, mc) in movables.iter_mut() {
            let delta_time = time.delta_seconds();
            match mc {
                Some(s) => m.apply_constrained_acc(&delta_time, s),
                None => m.apply_acc(&delta_time),
            }
            t.translation += m.vel.mul(delta_time).extend(0.);
            t.rotate_z(m.angular_vel * delta_time);
        }
    }
}
