mod player{
    use bevy::{prelude::*, math::vec2};
    use crate::{input::input::{ControlStatus, PlayerConstants}, movement::movement::Movable};

    struct Player;
    pub fn handle_player_inputs(
        input: Res<ControlStatus>,
        constants: Res<PlayerConstants>,
        players: Query<(&Transform, &mut Movable)>
    ){
        
        for (t, mut m) in players.iter(){
            if input.forward {
                let acc_to_add = t.rotation.xyz().truncate();
                
                //let foo = vec2(1, 2).
                //m.acc += player.
                // t.rotation.
                // m.acc = minnumf32(1., 2.)
            }
        }
    }
}