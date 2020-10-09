extern crate aqrun_lrbg;

use aqrun_lrbg::common;
use crossterm::event::{
    self, Event, KeyEvent, KeyCode
};

/// code refactor, modular
struct App {
    position_x: i32,
    position_y: i32,
    high: i32,
    width: i32,
}

impl App {
    pub fn startup() -> App {
        let high = 20;
        let width = 30;

        App {
            high,
            width,
            position_x: width / 2,
            position_y: high / 2,
        }
    }

    pub fn show(&self) {
        common::clear();

        for i in 0..self.high {
            for j in 0..self.width {
                if (i == self.position_y) && (j == self.position_x) {
                    print!(" * "); // 输出飞机
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    /// 与用户输入无关的更新
    pub fn update_witout_input(&self) {

    }

    /// 与用户输入有关的更新
    pub fn update_with_input(&mut self) {
        if let Event::Key(KeyEvent{ code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Char('a') => self.position_x -= 1,
                KeyCode::Char('w') => self.position_y -= 1,
                KeyCode::Char('s') => self.position_y += 1,
                KeyCode::Char('d') => self.position_x += 1,
                _ => {}
            }
        }
    }
}

pub fn main() {
    let mut app = App::startup();

    loop {
        app.show();
        app.update_witout_input();
        app.update_with_input();
    }
}