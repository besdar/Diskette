use super::{append_output, section};
use crate::i18n::text;
use gtk::prelude::*;
use gtk4 as gtk;

pub(crate) fn build_output_view() -> (gtk::TextView, gtk::TextBuffer) {
    let output_view = gtk::TextView::new();
    output_view.set_editable(false);
    output_view.set_cursor_visible(false);
    output_view.set_monospace(true);
    output_view.add_css_class("diskette-command-output");

    let output_buffer = output_view.buffer();
    append_output(&output_buffer, &format!("{}\n", text("diskette_ready")));

    (output_view, output_buffer)
}

pub(crate) fn build_output_group(output_view: &gtk::TextView) -> gtk::Box {
    let output_scroller = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_height(170)
        .child(output_view)
        .build();

    let output_group = section(text("command_output"));
    output_group.set_margin_bottom(12);
    output_group.append(&output_scroller);
    output_group
}
