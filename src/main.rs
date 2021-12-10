use std::error::Error;

use memflow_win32::Keyboard;

use crate::menu::Config;
mod sdk;
mod cheat;
pub mod menu;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting project membreak...");
    let offsets = sdk::offsets::get_offsets();

    let (mut process,
        mut kernel, 
        client_base, 
        engine_base) = cheat::setup();

    let keyboard = Keyboard::try_with(&mut kernel).unwrap();

    let config = Config { 
        aimbot_enable: true,
        aimbot_fov: 3.5f32,
        aimbot_smoothing: 8f32,
        recoil_control_amount: 0f32,
        recoil_control_enable: false,
        glow_enable: false,
        radar_enable: true,
    };

    println!("Cheat setup successful!");
    println!("Starting loop now...");
    
    loop {
        unsafe { 
            let hackloop_success = cheat::hack_loop(&mut  process, client_base, engine_base, &keyboard, &offsets, &config); 
            if !hackloop_success {
                break;
            }
        };

        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    Ok(())
}
