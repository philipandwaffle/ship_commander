pub mod propelled_object {
    use bevy::{
        prelude::{Component, Plugin, Query, Res, Transform, Vec2, Vec3},
        time::Time,
    };

    use crate::movement::movement::Movable;

    pub struct PropulsionPlugin;
    impl Plugin for PropulsionPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_system(propel_objects);
        }
    }

    pub struct PropulsionConstraints {
        // max movement acceleration
        pub max_acc: f32,
        // max movement velocity
        pub max_vel: f32,
        //max angular acceleration
        pub max_a_acc: f32,
        //max angular velocity
        pub max_a_vel: f32,
    }

    pub struct PropulsionValues {
        // forward acceleration
        pub f_acc: f32,
        // backward acceleration
        pub b_acc: f32,
        // left angular acceleration
        pub a_acc: f32,
    }

    trait Propulsion {
        fn apply_prop(&self, m: &mut Movable, dir_vec: Vec2, delta_time: &f32);
    }

    pub enum TranslationInput {
        Nothing,
        Forward,
        Backward,
    }
    pub enum RotationInput {
        Nothing,
        Left,
        Right,
    }

    #[derive(Component)]
    pub struct Ship {
        // the values that control the max translational and roational movement
        pub pc: PropulsionConstraints,
        // the values that describe the rate of translational and roational movement
        pub pv: PropulsionValues,
        pub ti: TranslationInput,
        pub ri: RotationInput,
    }

    impl Propulsion for Ship {
        fn apply_prop(&self, m: &mut Movable, n_dir_vec: Vec2, dt: &f32) {
            match self.ti {
                TranslationInput::Nothing => {
                    m.stop_acc();
                    m.add_dt_acc(-m.vel, dt)
                }
                TranslationInput::Forward => m.add_dt_acc(n_dir_vec * self.pv.f_acc, dt),
                TranslationInput::Backward => m.add_dt_acc(-n_dir_vec * self.pv.b_acc, dt),
            }
            if m.acc.length() > self.pc.max_acc {
                m.acc = self.pc.max_acc * m.acc.normalize();
            }
            if m.vel.length() > self.pc.max_vel {
                m.vel = self.pc.max_vel * m.vel.normalize();
            }

            match self.ri {
                RotationInput::Nothing => {
                    m.stop_angular_acc();
                    m.angular_vel -= m.angular_vel * dt * 0.9;
                }
                RotationInput::Left => m.add_dt_angular_acc(self.pv.a_acc, dt),
                RotationInput::Right => m.add_dt_angular_acc(-self.pv.a_acc, dt),
            }
            m.angular_acc = m.angular_acc.clamp(-self.pc.max_a_acc, self.pc.max_a_acc);
            m.angular_vel = m.angular_vel.clamp(-self.pc.max_a_vel, self.pc.max_a_vel);
        }
    }

    #[derive(Component)]
    struct Projectile {}

    fn propel_objects(
        time: Res<Time>,
        mut ships: Query<(&mut Movable, &Transform, &Ship)>,
        //projectiles: Query<(&mut Movable, &Transform, &Projectile)>,
    ) {
        let dt = time.delta_seconds();
        for (mut m, t, s) in ships.iter_mut() {
            let orientation = t.rotation.mul_vec3(Vec3::Y).truncate().normalize();
            s.apply_prop(&mut m, orientation, &dt)
        }
    }
}
