extern crate aqrun_lrbg;

use aqrun_lrbg::common;
use crossterm::event::{
    self, Event, KeyCode, KeyEvent
};

/// add a movable enemy
struct App {
    pub high: i32,
    pub width: i32,
    pub position_x: i32,
    pub position_y: i32,
    pub bullet_x: i32,
    pub bullet_y: i32,
    pub enemy_x: i32,
    pub enemy_y: i32,

    pub speed: i32,
}

impl App {
    pub fn new(high: i32, width: i32) -> App {
        let position_x = width / 2;
        let position_y = high / 2;

        App {
            high,
            width,
            position_x,
            position_y,
            bullet_x: position_x,
            bullet_y: -1,
            enemy_x: position_x,
            enemy_y: 0,

            speed: 0,
        }
    }

    pub fn show(&self) {
        common::clear();
        
        for i in 0..(self.high + 1) {
            for j in 0..(self.width + 1){
                if (i == self.position_y) && (j == self.position_x) {
                    print!("*"); // 输出飞机
                } else if (i == self.bullet_y) && (j == self.bullet_x) {
                    print!("|"); // 输出子弹
                } else if (i == self.enemy_y) && (j == self.enemy_x) {
                    print!("@"); // 输入敌机
                } else if i == self.high {
                    print!("-");
                } else if j == self.width {
                    print!("|");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    pub fn update_without_input(&mut self) {
        if self.bullet_y > -1 {
            self.bullet_y -= 1;
        }

        if self.speed < 5 {
            self.speed += 1;
        } else {
            self.enemy_y += 1;
            self.speed = 0;
        }

    }

    pub fn update_with_input(&mut self) {
        if let Event::Key(KeyEvent{ code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Char('a') => self.position_x -= 1,
                KeyCode::Char('w') => self.position_y -= 1,
                KeyCode::Char('s') => self.position_y += 1,
                KeyCode::Char('d') => self.position_x += 1,
                KeyCode::Char(' ') => {
                    self.bullet_y = self.position_y - 1;
                    self.bullet_x = self.position_x;
                },
                _ => {}
            }
        }
    }
}

pub fn main() {
    let mut app = App::new(20, 30);

    loop {
        app.show();
        app.update_without_input();
        app.update_with_input();
    }
}