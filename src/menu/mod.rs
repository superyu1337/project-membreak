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

pub fn draw_menu(config: &Config) -> Result<(), Box<dyn Error>> {
    stdout()
        .execute(Clear(ClearType::All))?
        .execute(SetForegroundColor(Color::Black))?
        .execute(SetBackgroundColor(Color::White))?
        .execute(Print("Amogus here"))?
        .execute(ResetColor)?;

    Ok(())
}