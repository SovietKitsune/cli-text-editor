#![allow(dead_code)]

extern crate rustbox;

use rustbox::{Color, Key};

enum Align {
    Left,
    Right,
}

fn pad(text: String, len: usize, align: Align) -> String {
    match align {
        Align::Right => format!("{}{}", String::from(' ').repeat(len - text.len()), text),
        Align::Left => format!("{}{}", text, String::from(' ').repeat(len - text.len())),
    }
}

// TODO; Give TextBox its own file
pub struct TextBox {
    text: String,
    cursor_x: isize,
    cursor_y: isize,
    x: usize,
    y: usize,
}

impl TextBox {
    pub fn new(text: Option<String>) -> TextBox {
        TextBox {
            text: text.unwrap_or("".to_string()),
            cursor_x: 0,
            cursor_y: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn render(&mut self, rustbox: &rustbox::RustBox) {
        self.update_pos();

        let lines = self.text.split('\n').count();
        let width = lines.to_string().len();

        rustbox.clear();

        rustbox.set_cursor(self.cursor_x + width as isize + 3, self.cursor_y);

        let to_iter = &self
            .text
            .split('\n')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        for (i, part) in to_iter.iter().enumerate() {
            rustbox.print(
                self.x,
                self.y + i,
                rustbox::RB_NORMAL,
                Color::White,
                Color::Default,
                &format!(
                    "{} | {}",
                    pad((i + 1).to_string(), width, Align::Right),
                    part
                ),
            );
        }

        rustbox.present();
    }

    pub fn update_pos(&mut self) {
        self.cursor_y = self.text.matches('\n').count() as isize;
        self.cursor_x = self.text.split('\n').last().unwrap_or(&self.text).len() as isize;
    }

    pub fn key_event(&mut self, pressed: Key) {
        match pressed {
            Key::Backspace => {
                self.text.pop();
            }
            Key::Enter => self.text.push('\n'),
            Key::Tab => self.text.push('\t'),
            Key::Char(c) => self.text.push(c),
            _ => {}
        };
    }
}
