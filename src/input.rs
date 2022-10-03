pub mod input {
    use bevy::{prelude::{KeyCode, ResMut, Res, Input, Plugin}, render::texture::ImagePlugin, reflect::erased_serde::__private::serde::forward_to_deserialize_any};

    pub struct InputPlugin;
    impl Plugin for InputPlugin{
        fn build(&self, app: &mut bevy::prelude::App) {
            app
                .insert_resource(Bindings::default())
                .insert_resource(ControlStatus::default())
                .insert_resource(PlayerConstants::default())
                .add_system(process_input);
        }
    }

    pub struct PlayerConstants{
        forward_acc: f32,
        reverse_acc: f32,
        max_forward_acc: f32,
        max_backward_acc: f32,
        max_forward_vel: f32,
        max_reverse_vel: f32,
        angular_acc: f32,
        max_angular_acc: f32,
        max_angular_vel: f32,
    }
    impl Default for PlayerConstants{
        fn default() -> Self {
            Self { 
                forward_acc: 5.,
                reverse_acc: 10.,
                max_forward_acc: 3.,
                max_backward_acc: 4.,
                max_forward_vel: 10.,
                max_reverse_vel: 5.,
                angular_acc: 2.,
                max_angular_acc: 1.,
                max_angular_vel: 4.,
            }
        }
    }
    
    pub struct Bindings{
        forward: KeyCode,
        backward: KeyCode,
        rotate_right: KeyCode,
        rotate_left: KeyCode,
        shoot: KeyCode,
    }
    impl Default for Bindings{
        fn default() -> Self {
            Self { 
                forward: KeyCode::W, 
                backward: KeyCode::S, 
                rotate_right: KeyCode::A, 
                rotate_left: KeyCode::D, 
                shoot: KeyCode::Space             
            }
        }
    }

    pub struct ControlStatus{
        pub forward: bool,
        pub backward: bool,
        pub rotate_right: bool,
        pub rotate_left: bool,
        pub shoot: bool,
    }
    impl Default for ControlStatus{
        fn default() -> Self {
            Self { 
                forward: Default::default(), 
                backward: Default::default(), 
                rotate_right: Default::default(), 
                rotate_left: Default::default(), 
                shoot: Default::default() 
            }
        }
    }

    pub fn process_input(
        input: Res<Input<KeyCode>>,
        bindings: Res<Bindings>,
        mut control_status: ResMut<ControlStatus>,
    ){
        control_status.forward = input.pressed(bindings.forward);
        control_status.backward = input.pressed(bindings.backward);
        control_status.rotate_right = input.pressed(bindings.rotate_right);
        control_status.rotate_left = input.pressed(bindings.rotate_left);
        control_status.shoot = input.pressed(bindings.shoot);
    }
}
