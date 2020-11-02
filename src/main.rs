extern crate rustbox;

use std::default::Default;

use rustbox::{Key, RustBox};

mod types;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut text_box = types::TextBox::new(Some("hello".to_string()));

    rustbox.present();
    loop {
        text_box.render(&rustbox);
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => match key {
                Key::Esc => break,
                key => text_box.key_event(key),
            },
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}
