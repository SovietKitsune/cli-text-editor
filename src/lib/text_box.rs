#![allow(dead_code)]

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

fn clamp<T>(n: T, min: T, max: T) -> T
where
    T: PartialOrd<T>,
{
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

pub struct TextBox {
    /// The text stored within the textbox
    pub text: String,
    /// The x position to draw the box
    pub x: usize,
    /// The y position to draw the box
    pub y: usize,
    /// The amount of lines to render
    pub lines: usize,
    /// To automatically update the max lines
    pub auto_update: bool,
    /// The position of the cursor in the x direction
    cursor_x: isize,
    /// The position of the cursor in the y direction
    cursor_y: isize,
}

// TODO; Split into traits

impl TextBox {
    pub fn new(text: Option<String>, lines: Option<usize>, auto_update: Option<bool>) -> TextBox {
        TextBox {
            text: text.unwrap_or("".to_string()),
            lines: lines.unwrap_or(5),
            auto_update: auto_update.unwrap_or(false),
            cursor_x: 0,
            cursor_y: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn render(&mut self, rustbox: &rustbox::RustBox) {
        self.update_pos();

        if self.auto_update {
            self.update_lines(rustbox);
        }

        let lines = self.text.split('\n').count();
        let width = lines.to_string().len();

        rustbox.clear();

        rustbox.set_cursor(
            self.cursor_x + width as isize + 3,
            clamp(self.cursor_y, 0, self.lines as isize),
        );

        let vec = self.to_vec();

        let starting = clamp(self.cursor_y - self.lines as isize, 0, vec.len() as isize) as usize;
        let ending = clamp(self.cursor_y + self.lines as isize, 0, vec.len() as isize) as usize;

        let to_iter = &vec[starting..ending];

        for (i, part) in to_iter.iter().enumerate() {
            rustbox.print(
                self.x,
                self.y + i,
                rustbox::RB_NORMAL,
                Color::White,
                Color::Default,
                &format!(
                    "{} | {}",
                    pad((i + 1 + starting).to_string(), width, Align::Right),
                    part
                ),
            );
        }

        rustbox.present();
    }

    pub fn update_pos(&mut self) {
        self.update_x();
        self.update_y();
    }

    pub fn update_lines(&mut self, rustbox: &rustbox::RustBox) {
        self.lines = rustbox.height();
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

    fn update_y(&mut self) {
        self.cursor_y = clamp(self.cursor_y, 0, self.max_y() as isize);
    }

    fn update_x(&mut self) {
        self.cursor_x = clamp(self.cursor_x, 0, self.max_x() as isize);
    }

    fn to_vec(&self) -> Vec<String> {
        self.text
            .split('\n')
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }

    fn to_vec_chars(&self) -> Vec<char> {
        self.text.chars().collect()
    }

    fn get_pos(&self) -> isize {
        let mut pos = 0;

        for string in self.to_vec().iter().take(self.cursor_y as usize) {
            pos += string.len() + 1
        }

        pos as isize + self.cursor_x
    }

    fn len_of_line(&self, y: isize) -> isize {
        self.to_vec()[y as usize].len() as isize
    }

    pub fn key_event(&mut self, pressed: Key) {
        match pressed {
            Key::Backspace => {
                if self
                    .to_vec_chars()
                    .get(clamp(self.get_pos() - 1, 0, self.to_vec_chars().len() as isize) as usize)
                    .is_some()
                {
                    self.text.remove(clamp(
                        self.get_pos() - 1,
                        0,
                        self.to_vec_chars().len() as isize,
                    ) as usize);
                }

                self.cursor_x = clamp(self.cursor_x - 1, 0, self.max_x() as isize)
            }
            Key::Enter => {
                self.text.insert(self.get_pos() as usize, '\n');

                self.cursor_y += 1;
            }
            Key::Tab => {
                self.text.insert(self.get_pos() as usize, '\t');

                self.cursor_x += 1;
            }
            Key::Down => self.cursor_y = clamp(self.cursor_y + 1, 0, self.max_y() as isize),
            Key::Up => self.cursor_y = clamp(self.cursor_y - 1, 0, self.max_y() as isize),
            Key::Left => self.cursor_x = clamp(self.cursor_x - 1, 0, self.max_x() as isize),
            Key::Right => self.cursor_x = clamp(self.cursor_x + 1, 0, self.max_x() as isize),
            Key::Char(c) => {
                self.text.insert(self.get_pos() as usize, c);

                self.cursor_x += 1;
            }
            _ => {}
        };
    }
}
