use std::error::Error;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use memflow_win32::Keyboard;
use std::thread;

use crate::menu::{Config, handle_menu};
mod sdk;
mod cheat;
pub mod menu;

static mut CONFIG: Config = Config { 
    aimbot_enable: false,
    aimbot_fov: 6f32,
    aimbot_smoothing: 10f32,
    recoil_control_amount: 30f32,
    recoil_control_enable: false,
    glow_enable: false,
    radar_enable: false,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting project membreak...");
    let offsets = sdk::offsets::get_offsets();

    let (mut process,
        mut kernel, 
        client_base, 
        engine_base) = cheat::setup();

    let keyboard = Keyboard::try_with(&mut kernel).unwrap();

    let mut config = unsafe { CONFIG.clone() };

    println!("Cheat setup successful!");
    println!("Starting loop now...");

    let (tx, rx) = std::sync::mpsc::channel();

    let menu_thread = thread::spawn(move || {
        enable_raw_mode().unwrap();
        loop {
            let result = handle_menu(&mut config).unwrap();

            if result == false {
                tx.send(Err("menu quit")).unwrap();
                disable_raw_mode().unwrap();
                break;
            }

            tx.send(Ok(config.clone())).unwrap();
            thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    loop {
        unsafe { 
            let cfg = CONFIG.clone();
            let new_cfg = rx.recv_timeout(std::time::Duration::from_nanos(500));

            if new_cfg.is_ok() {
                let new_cfg = new_cfg.unwrap();
                if new_cfg.is_ok() {
                    let cfg = new_cfg.unwrap();
                    CONFIG = cfg;
                } else {
                    break;
                }
            }


            let hackloop_success = cheat::hack_loop(&mut  process, client_base, engine_base, &keyboard, &offsets, &cfg); 
            if !hackloop_success {
                break;
            }
        };

        thread::sleep(std::time::Duration::from_nanos(500));
    }

    menu_thread.join().unwrap();
    Ok(())
}
