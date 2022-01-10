use std::{error::Error, io::{stdout}};

use crossterm::{ExecutableCommand, terminal::{Clear, ClearType}, event::{Event, read, poll, KeyEvent, KeyCode}};
use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub glow_enable: bool,
    pub chams_enable: bool,
    pub chams_brightness: f32,
    pub radar_enable: bool,
    pub aimbot_enable: bool,
    pub aimbot_fov: f32,
    pub aimbot_smoothing: f32,
    pub recoil_control_enable: bool,
    pub recoil_control_amount: f32,
    pub rage_mode: bool,
}

static mut CURRENT_TAB: i8 = 0;
static mut CURRENT_SELECTED: i8 = 0;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn draw_checkbox(name: &str, value: &mut bool, id: &mut i8, input_code: usize) {

    if (input_code == 3 || input_code == 4) && unsafe { CURRENT_SELECTED == *id }{
        *value = !*value;
    }

    let prefix: &str = unsafe { if CURRENT_SELECTED == *id { "->" } else { "  " } };

    print!("{} {} - {}\n\r", prefix, name, value);
    *id += 1;
}

fn draw_slider(name: &str, value: &mut f32, id: &mut i8, start: f32, end: f32, step: f32, suffix: &str, input_code: usize) {

    if input_code == 3 && unsafe { CURRENT_SELECTED == *id } {
        if *value + step <= end {
            *value += step;
        }
    } else if input_code == 4 && unsafe { CURRENT_SELECTED == *id } {
        if *value - step >= start {
            *value -= step;
        }
    }

    let prefix: &str = unsafe { if CURRENT_SELECTED == *id { "->" } else { "  " } };

    print!("{} {} - {}{}\n\r", prefix, name, value, suffix);
    *id += 1;
}

pub fn draw_tab_menu(value: &mut i8, tabs: Vec<String>, id: &mut i8, input_code: usize) {
    let mut counter: i8 = 0;

    let prefix: &str = unsafe { if CURRENT_SELECTED == *id { "-> " } else { "   " } };
    print!("{}", prefix);

    if input_code == 3 && unsafe { CURRENT_SELECTED == *id } {
        if *value + 1 <= tabs.len() as i8 - 1i8 {
            *value += 1;
        }
    } else if input_code == 4 && unsafe { CURRENT_SELECTED == *id } {
        if *value - 1 >= 0 {
            *value -= 1;
        }
    }

    for tab_name in tabs {
        if counter == *value {
            print!("[{}] ", tab_name);
        } else {
            print!("{} " , tab_name);
        }
        counter += 1;
    }
    print!("\n\r");
    print!("===============================================\n\r");

    *id += 1;
}

pub fn handle_menu(config: &mut Config) -> Result<bool, Box<dyn Error>> {
    let mut id = 0;
    let mut input_code: usize = 0; 
    // 0 - nothing
    // 1 - go up
    // 2 - go down
    // 3 - increase or swap bool
    // 4 - decrease or swap bool

    if poll(std::time::Duration::from_micros(500))? {
        let event: Result<KeyEvent, bool> = match read()? {
            Event::Key(event) => Ok(event),
            _ => Err(true)
        };

        if event.is_ok() {
            let event: KeyEvent = event.unwrap();

            match event.code {
                KeyCode::Char('q') => { return Ok(false) }
                KeyCode::Down => unsafe { 
                    CURRENT_SELECTED += 1
                }
                KeyCode::Up => unsafe { 
                    if CURRENT_SELECTED - 1 >= 0 {
                        CURRENT_SELECTED -= 1
                    }
                }
                KeyCode::Right => { input_code = 3 }
                KeyCode::Left => { input_code = 4 }
                _ => {}
            }
        }
    }

    let mut out = stdout();

    out
        .execute(Clear(ClearType::All)).unwrap();

    print!("Project Membreak {} | made with ♡ by superyu\n\r", VERSION);
    print!("===============================================\n\r");

    unsafe { 
        draw_tab_menu(&mut CURRENT_TAB, 
            vec!["General".to_owned(), "Danger Zone".to_owned()], 
            &mut id, input_code) 
        };

    if unsafe { CURRENT_TAB == 0 } {
        draw_checkbox("Glow", &mut config.glow_enable, &mut id, input_code);
        draw_checkbox("Chams", &mut config.chams_enable, &mut id, input_code);
        draw_slider("Chams Brightness", &mut config.chams_brightness, &mut id, 0f32, 100f32, 10f32, "%", input_code);
        draw_checkbox("Radar", &mut config.radar_enable, &mut id, input_code);
    
        draw_checkbox("Aimbot", &mut config.aimbot_enable, &mut id, input_code);
        draw_slider("Aimbot FOV", &mut config.aimbot_fov, &mut id, 0f32, 180f32, 0.25f32, "°", input_code);
        draw_slider("Aimbot Smoothing", &mut config.aimbot_smoothing, &mut id, 0f32, 15f32, 0.25f32, "",input_code);
    
        draw_checkbox("Recoil Control System", &mut config.recoil_control_enable, &mut id, input_code);
        draw_slider("Recoil Control Amount", &mut config.recoil_control_amount, &mut id, 0f32, 100f32, 5f32, "%", input_code);
    } else {
        draw_checkbox("Ragemode (No smoothing, goes thru walls)", &mut config.rage_mode, &mut id, input_code);
    }
    print!("==============================================\n\r");
    print!("Press q to quit.\n\r");
    print!("==============================================\n\r");

    unsafe {
        if CURRENT_SELECTED >= id {
            CURRENT_SELECTED = id - 1;
        }
    }

    out.flush().unwrap();

    Ok(true)
}