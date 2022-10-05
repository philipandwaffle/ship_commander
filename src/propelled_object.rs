mod propelled_object {
    use crate::movement::movement::Movable;

    struct PropulsionConstraints {
        // max movement acceleration
        pub max_acc: f32,
        // max movement velocity
        pub max_vel: f32,
        //max angular acceleration
        pub max_a_acc: f32,
        //max angular velocity
        pub max_a_vel: f32,
    }

    struct PropulsionValues {
        // forward acceleration
        pub f_acc: f32,
        // backward acceleration
        pub b_acc: f32,
        // right angular acceleration
        pub r_a_acc: f32,
        // left angular acceleration
        pub l_a_acc: f32,
    }

    //
    trait Propulsion {
        fn apply_prop(&self, m: &mut Movable, delta_time: &f32);
    }

    enum TranslationInput {
        Nothing,
        Forward,
        Backward,
    }
    enum RotationInput {
        Nothing,
        Left,
        Right,
    }
    struct Ship {
        // the values that control the max translational and roational movement
        pub pc: PropulsionConstraints,
        // the values that describe the rate of translational and roational movement
        pub pv: PropulsionValues,
        pub ti: TranslationInput,
        pub ri: RotationInput,
    }

    impl Propulsion for Ship {
        fn apply_prop(&self, m: &mut Movable, delta_time: &f32) {
            match self.ti {
                TranslationInput::Nothing => m.stop_acc(),
                TranslationInput::Forward => ,
                TranslationInput::Backward => todo!(),
            }
            todo!()
        }
    }

    struct Projectile {}
    fn propel_objects() {}
}
