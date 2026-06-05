#[cfg(feature = "gui")]
pub mod gui;

pub mod core;

#[macro_use]
extern crate log;

use static_init::dynamic;
use std::path::PathBuf;

#[cfg(not(target_os = "android"))]
use crate::core::utils::setup_uad_dir;

#[cfg(not(target_os = "android"))]
#[dynamic]
pub static CONFIG_DIR: PathBuf = setup_uad_dir(dirs::config_dir());

#[cfg(target_os = "android")]
#[dynamic]
pub static CONFIG_DIR: PathBuf =
    PathBuf::from("/data/data/com.github.uadgui_debloater/files/uad");

#[cfg(not(target_os = "android"))]
#[dynamic]
pub static CACHE_DIR: PathBuf = setup_uad_dir(dirs::cache_dir());

#[cfg(target_os = "android")]
#[dynamic]
pub static CACHE_DIR: PathBuf =
    PathBuf::from("/data/data/com.github.uadgui_debloater/cache/uad");

use fern::{
    colors::{Color, ColoredLevelConfig},
    FormatCallback,
};
use log::Record;
use std::{fmt::Arguments, fs::OpenOptions};

pub fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new().info(Color::Green);

    let make_formatter = |use_colors: bool| {
        move |out: FormatCallback, message: &Arguments, record: &Record| {
            out.finish(format_args!(
                "{} {} [{}:{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                if use_colors {
                    format!("{:5}", colors.color(record.level()))
                } else {
                    format!("{:5}", record.level().to_string())
                },
                record.file().unwrap_or("?"),
                record.line().map(|l| l.to_string()).unwrap_or_default(),
                message
            ));
        }
    };

    let default_log_level = log::LevelFilter::Warn;
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .truncate(false)
        .open(CACHE_DIR.join(format!(
            "UAD_{}.log",
            chrono::Local::now().format("%Y%m%d")
        )))?;

    let file_dispatcher = fern::Dispatch::new()
        .format(make_formatter(false))
        .level(default_log_level)
        .level_for("uad_gui", log::LevelFilter::Debug)
        .chain(log_file);

    let stdout_dispatcher = fern::Dispatch::new()
        .format(make_formatter(true))
        .level(default_log_level)
        .level_for("uad_gui", log::LevelFilter::Warn)
        .chain(std::io::stdout());

    fern::Dispatch::new()
        .chain(stdout_dispatcher)
        .chain(file_dispatcher)
        .apply()?;

    Ok(())
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_github_uadgui_debloater_NativeBridge_runDeBloater(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    list_path: jni::sys::JString,
) -> jni::sys::JString {
    let url: String = match env.get_string(&list_path) {
        Ok(s) => s.into(),
        Err(e) => {
            return env
                .new_string(format!("error: failed to read path: {}", e))
                .unwrap()
                .into_raw()
        }
    };

    match crate::core::uad_lists::get_list_from_net(&url, None) {
        Ok(packages) => {
            let msg = format!("loaded {} package definitions", packages.len());
            env.new_string(msg).unwrap().into_raw()
        }
        Err(e) => env.new_string(format!("error: {}", e)).unwrap().into_raw(),
    }
}
