use gtk::prelude::*;
use gtk4 as gtk;

pub(crate) fn append_output(buffer: &gtk::TextBuffer, text: &str) {
    let mut end = buffer.end_iter();
    buffer.insert(&mut end, text);
    if !text.ends_with('\n') {
        buffer.insert(&mut end, "\n");
    }
}
