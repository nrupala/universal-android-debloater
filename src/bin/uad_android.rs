#[macro_use]
extern crate log;

mod core;

use static_init::dynamic;
use std::path::PathBuf;
use std::process::Command;

#[dynamic]
static CONFIG_DIR: PathBuf = PathBuf::from("/data/data/com.github.uadgui_debloater/files/uad");

#[dynamic]
static CACHE_DIR: PathBuf = PathBuf::from("/data/data/com.github.uadgui_debloater/cache/uad");

fn main() {
    println!("UAD Android Debloater");
    match run() {
        Ok(()) => println!("Done."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = core::config::Config::load_configuration_file();
    eprintln!("config file loaded");

    let packages = core::uad_lists::get_list_from_net(
        config.general.uad_url.unwrap_or_else(|| "https://raw.githubusercontent.com/0x192/universal-android-debloater/main/resources/assets/uad_lists.json".into()),
        None,
    )?;
    eprintln!("loaded {} package definitions", packages.len());

    Ok(())
}

/// JNI entry point — called from Android (Kotlin/Java).
/// Signature matches: `native fun runDeBloater(listPath: String): String`
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_github_uadgui_debloater_NativeBridge_runDeBloater(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    list_path: jni::sys::jstring,
) -> jni::sys::jstring {
    match run() {
        Ok(()) => jni::objects::JString::from("ok"),
        Err(e) => jni::objects::JString::from(format!("error: {}", e)),
    }
    .into_raw()
}
