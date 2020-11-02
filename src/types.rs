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

fn clamp(n: i32, min: i32, max: i32) -> i32 {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
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
        // self.cursor_x = self.text.split('\n').last().unwrap_or(&self.text).len() as isize;
        self.update_x();
        self.update_y();
    }

    pub fn max_y(&self) -> usize {
        self.text.matches('\n').count()
    }

    pub fn max_x(&self) -> usize {
        self.text
            .split('\n')
            .nth(self.cursor_y as usize)
            .unwrap_or(&self.text)
            .len()
    }

    pub fn update_y(&mut self) {
        self.cursor_y = clamp(self.cursor_y as i32, 0, self.max_y() as i32) as isize;
    }

    pub fn update_x(&mut self) {
        self.cursor_x = clamp(self.cursor_x as i32, 0, self.max_x() as i32) as isize;
    }

    pub fn key_event(&mut self, pressed: Key) {
        match pressed {
            Key::Backspace => {
                self.text.pop();
                // self.update_y();
            }
            Key::Enter => {
                self.text.push('\n');
                self.cursor_y += 1;
            }
            Key::Tab => self.text.push('\t'),
            Key::Down => {
                self.cursor_y = clamp((self.cursor_y + 1) as i32, 0, self.max_y() as i32) as isize
            }
            Key::Up => {
                self.cursor_y = clamp((self.cursor_y - 1) as i32, 0, self.max_y() as i32) as isize
            }
            Key::Left => {
                self.cursor_x = clamp((self.cursor_x - 1) as i32, 0, self.max_x() as i32) as isize
            }
            Key::Right => {
                self.cursor_x = clamp((self.cursor_x + 1) as i32, 0, self.max_x() as i32) as isize
            }
            Key::Char(c) => self.text.push(c),
            _ => {}
        };
    }
}
