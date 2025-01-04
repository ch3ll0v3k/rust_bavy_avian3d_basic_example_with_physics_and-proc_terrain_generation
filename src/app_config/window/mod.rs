// prettier-ignore

use std::{fs, sync::MutexGuard}; // ::{self, read_to_string};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use bevy::prelude::Resource;
use serde::{ Deserialize, Serialize };
// prettier-ignore

use crate::{ app_config::read_file_to_string, sys_paths };

// prettier-ignore
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct UseWinSize {
  pub x: f32,
  pub y: f32
}

// prettier-ignore
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct WinPosition {
  pub x: f32,
  pub y: f32
}

// prettier-ignore
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct WindowConfig {
  pub scale_factor_override: f32,
  pub use_auto_vsyn: bool,
  pub use_fullscreen: bool,
  pub resizable: bool,
  pub use_win_size: UseWinSize,
  pub position: WinPosition,
}

static C_TYPE_T: &str = "#Window:";
static DEBUG_CONFIG: Lazy<Mutex<Option<WindowConfig>>> = Lazy::new(|| Mutex::new(None));

// prettier-ignore
pub fn config() -> WindowConfig {

  // {
  //   // Same as below 
  //   let mut config = DEBUG_CONFIG.lock().unwrap();
  //   config.get_or_insert_with(|| load_config());
  //   config.clone().unwrap()      
  // }

  let mut config: MutexGuard<'_, Option<WindowConfig>> = DEBUG_CONFIG.lock().unwrap();
  if config.is_none() {
    // dbgln!("config({C_TYPE_T}) loading...");
    let cfg: WindowConfig = load_config();
    *config = Some(cfg);
  }else{
    // dbgln!("config({C_TYPE_T}) already loaded...");
  }
  config.clone().unwrap()
  
}

fn load_config() -> WindowConfig {
  let path: &str = sys_paths::config::EConfig::Window.as_str();
  // dbgln!("load_config({C_TYPE_T}): load config: @ {}", path);
  let utf8_raw = read_file_to_string(path);
  let config: WindowConfig = serde_json
    ::from_str(&utf8_raw)
    .expect(format!("load_config({C_TYPE_T}): failed to parse config file: ({})", path).as_str());
  dbgln!("load_config({C_TYPE_T}): loaded config: {:#?}", config);
  config
}
