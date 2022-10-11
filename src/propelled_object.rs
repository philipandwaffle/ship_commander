pub mod propelled_object {
    use std::sync::Arc;

    use bevy::{
        prelude::{Component, Plugin, Query, Res, Transform, Vec2, Vec3},
        time::Time,
    };

    use crate::{movement::movement::Movable, ship_controller::controllers::ShipController};

    pub struct PropulsionPlugin;
    impl Plugin for PropulsionPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_system(propel_projectiles).add_system(propel_ships);
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
    impl Clone for PropulsionConstraints {
        fn clone(&self) -> Self {
            Self {
                max_acc: self.max_acc.clone(),
                max_vel: self.max_vel.clone(),
                max_a_acc: self.max_a_acc.clone(),
                max_a_vel: self.max_a_vel.clone(),
            }
        }
    }

    pub struct PropulsionValues {
        // forward acceleration
        pub f_acc: f32,
        // backward acceleration
        pub b_acc: f32,
        // left angular acceleration
        pub a_acc: f32,
    }
    impl Clone for PropulsionValues {
        fn clone(&self) -> Self {
            Self {
                f_acc: self.f_acc.clone(),
                b_acc: self.b_acc.clone(),
                a_acc: self.a_acc.clone(),
            }
        }
    }

    trait Propulsion {
        fn apply_prop(&self, m: &mut Movable, n_dir_vec: Vec2, dt: &f32);
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
    pub enum WeaponState {
        Idle,
        Cooldown(f32),
        Firing,
    }

    #[derive(Component)]
    pub struct Ship {
        // the values that control the max translational and roational movement
        pub pc: PropulsionConstraints,
        // the values that describe the rate of translational and roational movement
        pub pv: PropulsionValues,
        pub ti: TranslationInput,
        pub ri: RotationInput,
        pub ammo: Vec<Projectile>,
        pub controller: Option<Arc<dyn ShipController + Sync + Send>>,
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
    pub struct Projectile {
        // the values that control the max translational and roational movement
        pub pc: PropulsionConstraints,
        // the values that describe the rate of translational and roational movement
        pub pv: PropulsionValues,
    }
    impl Propulsion for Projectile {
        fn apply_prop(&self, m: &mut Movable, n_dir_vec: Vec2, dt: &f32) {
            m.add_dt_acc(n_dir_vec * self.pv.f_acc, dt);
            if m.acc.length() > self.pc.max_acc {
                m.acc = self.pc.max_acc * m.acc.normalize();
            }
            if m.vel.length() > self.pc.max_vel {
                m.vel = self.pc.max_vel * m.vel.normalize();
            }

            m.add_dt_angular_acc(self.pv.a_acc, dt);
            m.angular_acc = m.angular_acc.clamp(-self.pc.max_a_acc, self.pc.max_a_acc);
            m.angular_vel = m.angular_vel.clamp(-self.pc.max_a_vel, self.pc.max_a_vel);
        }
    }
    impl Clone for Projectile {
        fn clone(&self) -> Self {
            Self {
                pc: self.pc.clone(),
                pv: self.pv.clone(),
            }
        }
    }

    fn propel_ships(time: Res<Time>, mut ships: Query<(&mut Movable, &Transform, &Ship)>) {
        let dt = time.delta_seconds();
        for (mut m, t, s) in ships.iter_mut() {
            let orientation = t.rotation.mul_vec3(Vec3::Y).truncate().normalize();
            s.apply_prop(&mut m, orientation, &dt)
        }
    }
    fn propel_projectiles(
        time: Res<Time>,
        mut projectiles: Query<(&mut Movable, &Transform, &Projectile)>,
    ) {
        let dt = time.delta_seconds();
        for (mut m, t, p) in projectiles.iter_mut() {
            let orientation = t.rotation.mul_vec3(Vec3::Y).truncate().normalize();
            p.apply_prop(&mut m, orientation, &dt)
        }
    }
}
