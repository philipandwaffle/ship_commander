pub mod input {
    use bevy::prelude::{Input, KeyCode, Plugin, Res, ResMut};

    pub struct InputPlugin;
    impl Plugin for InputPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.insert_resource(Bindings::default())
                .insert_resource(ControlStatus::default())
                .add_system(process_input);
        }
    }

    pub struct Bindings {
        forward: KeyCode,
        backward: KeyCode,
        rotate_right: KeyCode,
        rotate_left: KeyCode,
        shoot: KeyCode,
    }
    impl Default for Bindings {
        fn default() -> Self {
            Self {
                forward: KeyCode::W,
                backward: KeyCode::S,
                rotate_right: KeyCode::D,
                rotate_left: KeyCode::A,
                shoot: KeyCode::Space,
            }
        }
    }

    pub struct ControlStatus {
        pub forward: bool,
        pub backward: bool,
        pub rotate_right: bool,
        pub rotate_left: bool,
        pub shoot: bool,
    }
    impl Default for ControlStatus {
        fn default() -> Self {
            Self {
                forward: Default::default(),
                backward: Default::default(),
                rotate_right: Default::default(),
                rotate_left: Default::default(),
                shoot: Default::default(),
            }
        }
    }

    pub fn process_input(
        input: Res<Input<KeyCode>>,
        bindings: Res<Bindings>,
        mut control_status: ResMut<ControlStatus>,
    ) {
        control_status.forward = input.pressed(bindings.forward);
        control_status.backward = input.pressed(bindings.backward);
        control_status.rotate_right = input.pressed(bindings.rotate_right);
        control_status.rotate_left = input.pressed(bindings.rotate_left);
        control_status.shoot = input.just_pressed(bindings.shoot);
    }
}
