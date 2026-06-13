#![warn(unsafe_code)]

mod actions;
mod app;
mod components;
mod i18n;
mod models;
mod pages;
mod services;
mod settings;
mod utils;
mod window;

use gtk4::glib;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> glib::ExitCode {
    app::run()
}
