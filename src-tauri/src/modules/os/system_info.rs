use std::env;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub name: String,
    pub cpu_arch: String,
}

pub fn new() -> SystemInfo {
    let info = SystemInfo {
        name: env::consts::OS.to_string(),
        cpu_arch: env::consts::ARCH.to_string(),
    };
    info
}
