use gtk::glib::prelude::IsA;
use gtk::prelude::*;
use gtk4 as gtk;

pub(crate) fn page_box() -> gtk::Box {
    let page = gtk::Box::new(gtk::Orientation::Vertical, 14);
    page.set_margin_top(18);
    page.set_margin_bottom(18);
    page.set_margin_start(18);
    page.set_margin_end(18);
    page
}

pub(crate) fn section(title: &str) -> gtk::Box {
    let group = gtk::Box::new(gtk::Orientation::Vertical, 10);
    group.set_margin_start(12);
    group.set_margin_end(12);

    let label = gtk::Label::new(Some(title));
    label.set_xalign(0.0);
    label.add_css_class("heading");
    group.append(&label);
    group
}

pub(crate) fn field_row<W: IsA<gtk::Widget>>(label: &str, child: &W) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    row.set_hexpand(true);
    let row_label = gtk::Label::new(Some(label));
    row_label.set_xalign(0.0);
    row_label.set_width_chars(16);
    row_label.add_css_class("dim-label");
    row.append(&row_label);
    row.append(child);
    row
}
