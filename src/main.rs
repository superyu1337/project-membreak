use std::error::Error;

use memflow_win32::Keyboard;

use crate::menu::Config;
mod sdk;
mod cheat;
pub mod menu;

fn main() -> Result<(), Box<dyn Error>> {
    let offsets = sdk::offsets::get_offsets();

    let (mut process,
        mut kernel, 
        client_base, 
        engine_base) = cheat::setup();

    let keyboard = Keyboard::try_with(&mut kernel).unwrap();

    let config = Config { 
        aimbot_enable: false,
        aimbot_fov: 0f32,
        aimbot_smoothing: 0f32,
        recoil_control_amount: 0f32,
        recoil_control_enable: false,
        glow_enable: true,
    };
    
    loop {
        unsafe { cheat::hack_loop(&mut  process, &kernel, client_base, engine_base, &keyboard, &offsets, &config); }
    }

    Ok(())
}
