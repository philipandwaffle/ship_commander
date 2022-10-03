pub mod movement{
    use std::{ops::{Add, Mul}, f32::consts::PI};

    use bevy::prelude::*;
    #[derive(Component)]
    pub struct Movable {
        pub acc: Vec2,
        pub vel: Vec2,
        pub angular_acc: f32,
        pub angular_vel: f32,
    }
    impl Default for Movable{
        fn default() -> Self {
        Self { 
            acc: default(), 
            vel: default(),
            angular_acc: default(), 
            angular_vel: default(), 
        }
    }
    }
    impl Movable{
        pub fn apply_acc(&mut self, delta_time: &f32){
            self.vel += self.acc * *delta_time;
            self.angular_vel += self.angular_acc * *delta_time;
        }
    }

    pub fn movement(
        time: Res<Time>,
        mut movables: Query<(&mut Transform, &mut Movable)>
    ){
        for (mut t, mut m) in movables.iter_mut(){
            let delta_time = time.delta_seconds();
            m.apply_acc(&delta_time);
            t.translation += m.vel.mul(delta_time).extend(0.);
            t.rotate_z(m.angular_vel * delta_time);
            println!("{}", t.rotation.xyz().z*180.);
        }
    }
}