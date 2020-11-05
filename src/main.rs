extern crate rustbox;

use std::default::Default;

use rustbox::{InitOptions, Key, RustBox};

mod lib;

use lib::text_box::TextBox;

fn main() {
    let rustbox = match RustBox::init(InitOptions {
        buffer_stderr: true,
        ..Default::default()
    }) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut text_box = TextBox::new(
        Some(
            "
        Boxes are cool, yee!
        [][][][][][][][][][]
        Boxes are very cool!
        "
            .to_string(),
        ),
        Some(rustbox.height()),
        Some(true),
    );

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
