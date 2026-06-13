use crate::settings::APP_ID;
use gtk4::gio;
use gtk4::prelude::*;
use libadwaita as adw;

pub(crate) fn run() -> gtk4::glib::ExitCode {
    adw::init().expect("Libadwaita must initialize before the application starts");

    let app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_activate(|app| {
        crate::window::build(app);
    });

    app.set_resource_base_path(Some("/app/diskette/Diskette"));
    app.run()
}
