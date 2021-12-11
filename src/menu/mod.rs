use std::{error::Error, io::stdout};

use crossterm::{style::{SetForegroundColor, SetBackgroundColor, ResetColor, Color, Print}, ExecutableCommand, terminal::{Clear, ClearType}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub glow_enable: bool,
    pub radar_enable: bool,
    pub aimbot_enable: bool,
    pub aimbot_fov: f32,
    pub aimbot_smoothing: f32,
    pub recoil_control_enable: bool,
    pub recoil_control_amount: f32
}

static CURRENT_SELECTED: usize = 0;

fn draw_checkbox(name: &str, value: &bool, id: &mut usize) {
    println!("{} - {}", name, value);
    *id += 1;
}

fn draw_slider(name: &str, value: &f32, id: &mut usize, start: f32, end: f32, step: f32) {
    println!("{} - {}", name, value);
    *id += 1;
}

pub fn handle_menu(config: &Config) -> Result<(), Box<dyn Error>> {

    let mut id = 0;

    stdout()
        .execute(Clear(ClearType::All)).unwrap();

    draw_checkbox("Glow", &config.glow_enable, &mut id);
    draw_checkbox("Radar", &config.radar_enable, &mut id);

    draw_checkbox("Aimbot", &config.aimbot_enable, &mut id);
    draw_slider("Aimbot FOV", &config.aimbot_fov, &mut id, 0f32, 15f32, 0.25f32);
    draw_slider("Aimbot Smoothing", &config.aimbot_smoothing, &mut id, 0f32, 15f32, 0.25f32);

    draw_checkbox("Recoil Control System", &config.recoil_control_enable, &mut id);
    draw_slider("Recoil Control Amount", &config.recoil_control_amount, &mut id, 0f32, 100f32, 5f32);


    Ok(())
}