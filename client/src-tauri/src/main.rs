// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

fn main() {
    #[cfg(target_os = "linux")]
    env::set_var("GDK_BACKEND", "x11");
    #[cfg(target_os = "linux")]
    env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    friendlyfire_lib::run()
}
