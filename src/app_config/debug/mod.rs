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
pub struct DebugConfig {
  pub allowed_debug_physics: bool, //  = !true;
  pub allowed_debug_engine: bool, //  = true;
  pub allowed_debug_fps: bool, //  = true;
  pub is_wireframe_default_on: bool, //  = false;
  pub enable_world_inspector: bool, //  = false;
  pub measure_avg_fps_each: u32, //  = 15;
  pub fixed_pfs: f64, //  = 60.0;
}

static C_TYPE_T: &str = "#Debug:";
static DEBUG_CONFIG: Lazy<Mutex<Option<DebugConfig>>> = Lazy::new(|| Mutex::new(None));

// prettier-ignore
pub fn config() -> DebugConfig {

  // {
  //   // Same as below 
  //   let mut config = DEBUG_CONFIG.lock().unwrap();
  //   config.get_or_insert_with(|| load_config());
  //   config.clone().unwrap()      
  // }

  let mut config: MutexGuard<'_, Option<DebugConfig>> = DEBUG_CONFIG.lock().unwrap();
  if config.is_none() {
    // dbgln!("config({C_TYPE_T}) loading...");
    let cfg: DebugConfig = load_config();
    *config = Some(cfg);
  }else{
    // dbgln!("config({C_TYPE_T}) already loaded...");
  }
  config.clone().unwrap()
  
}

fn load_config() -> DebugConfig {
  let path: &str = sys_paths::config::EConfig::Debug.as_str();
  // dbgln!("load_config({C_TYPE_T}): load config: @ {}", path);
  let utf8_raw = read_file_to_string(path);
  let config: DebugConfig = serde_json
    ::from_str(&utf8_raw)
    .expect(format!("load_config({C_TYPE_T}): failed to parse config file: ({})", path).as_str());
  dbgln!("load_config({C_TYPE_T}): loaded config: {:#?}", config);
  config
}
