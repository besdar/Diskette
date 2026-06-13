use gtk4::gio;
use gtk4::glib;

pub(crate) fn open_uri(uri: &str) {
    let _ = open_uri_result(uri);
}

pub(crate) fn open_uri_result(uri: &str) -> Result<(), glib::Error> {
    gio::AppInfo::launch_default_for_uri(uri, None::<&gio::AppLaunchContext>)
}
