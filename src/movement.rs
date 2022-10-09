pub mod movement {
    use bevy::{
        prelude::{default, Component, Query, Res, Transform, Vec2},
        time::Time,
    };

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
        pub fn add_dt_acc(&mut self, acc: Vec2, dt: &f32) {
            self.acc += acc * dt.clone();
        }
        pub fn stop_acc(&mut self) {
            self.acc = Vec2::ZERO;
        }

        pub fn add_dt_angular_acc(&mut self, a_acc: f32, dt: &f32) {
            self.angular_acc += a_acc * dt;
        }
        pub fn stop_angular_acc(&mut self) {
            self.angular_acc = 0.;
        }

        pub fn apply_acc(&mut self, dt: &f32) {
            self.vel += self.acc * *dt;
            self.angular_vel += self.angular_acc * *dt;
        }
    }

    pub fn movement(time: Res<Time>, mut movables: Query<(&mut Transform, &mut Movable)>) {
        for (mut t, mut m) in movables.iter_mut() {
            let dt = time.delta_seconds();
            m.apply_acc(&dt);
            t.translation += (m.vel * dt).extend(0.);
            t.rotate_z(m.angular_vel * dt);
        }
    }
}
