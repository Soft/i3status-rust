#[cfg(feature = "time")] mod time;
#[cfg(feature = "template")] mod template;
#[cfg(feature = "load")] mod load;
#[cfg(feature = "memory")] mod memory;
#[cfg(feature = "cpu")] mod cpu;
#[cfg(feature = "music")] mod music;
#[cfg(feature = "battery")] mod battery;
#[cfg(feature = "custom")] mod custom;
#[cfg(feature = "disk_space")] mod disk_space;
#[cfg(feature = "pacman")] mod pacman;
#[cfg(feature = "temperature")] mod temperature;
#[cfg(feature = "toggle")] mod toggle;
#[cfg(feature = "sound")] mod sound;

#[cfg(feature = "time")] use self::time::*;
#[cfg(feature = "template")] use self::template::*;
#[cfg(feature = "load")] use self::music::*;
#[cfg(feature = "memory")] use self::cpu::*;
#[cfg(feature = "cpu")] use self::load::*;
#[cfg(feature = "music")] use self::memory::*;
#[cfg(feature = "battery")] use self::battery::*;
#[cfg(feature = "custom")] use self::custom::*;
#[cfg(feature = "disk_space")] use self::disk_space::*;
#[cfg(feature = "pacman")] use self::pacman::*;
#[cfg(feature = "temperature")] use self::sound::*;
#[cfg(feature = "toggle")] use self::toggle::*;
#[cfg(feature = "sound")] use self::temperature::*;

use super::block::Block;
use super::scheduler::Task;

extern crate serde_json;
extern crate dbus;

use serde_json::Value;
use std::sync::mpsc::Sender;

macro_rules! boxed ( { $b:expr } => { Box::new($b) as Box<Block> }; );

pub fn create_block(name: &str, config: Value, tx_update_request: Sender<Task>, theme: &Value) -> Box<Block> {
    match name {
        #[cfg(feature = "time")]
        "time" => boxed!(Time::new(config, theme.clone())),
        #[cfg(feature = "template")]
        "template" => boxed!(Template::new(config, tx_update_request, theme.clone())),
        #[cfg(feature = "music")]
        "music" => boxed!(Music::new(config, tx_update_request, theme)),
        #[cfg(feature = "load")]
        "load" => boxed!(Load::new(config, theme.clone())),
        #[cfg(feature = "memory")]
        "memory" => boxed!(Memory::new(config, tx_update_request, theme.clone())),
        #[cfg(feature = "cpu")]
        "cpu" => boxed!(Cpu::new(config, theme.clone())),
        #[cfg(feature = "pacman")]
        "pacman" => boxed!(Pacman::new(config, theme.clone())),
        #[cfg(feature = "battery")]
        "battery" => boxed!(Battery::new(config, theme.clone())),
        #[cfg(feature = "custom")]
        "custom" => boxed!(Custom::new(config, tx_update_request, theme.clone())),
        #[cfg(feature = "disk_space")]
        "disk_space" => boxed!(DiskSpace::new(config, theme.clone())),
        #[cfg(feature = "toggle")]
        "toggle" => boxed!(Toggle::new(config, theme.clone())),
        #[cfg(feature = "sound")]
        "sound" => boxed!(Sound::new(config, theme.clone())),
        #[cfg(feature = "temperature")]
        "temperature" => boxed!(Temperature::new(config, theme.clone())),
        _ => {
            panic!("Not a registered block: {}", name);
        }
    }
}
