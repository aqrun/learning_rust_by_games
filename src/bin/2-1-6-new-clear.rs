use std::io::{self, Write};
use std::time::Duration;
use crossterm::{
    cursor,
    event::{
        self, poll, Event, KeyCode, KeyEvent
    },
    execute,
    terminal,
};
use rand::prelude::*;

/// 新的清屏实现
/// 
/// * 目前画面闪烁严重
/// * 改进绘制完成将光标移动到原点位置，再重绘
/// * 实现类似清屏的功能
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
    pub score: i32,
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
            score: 0,
        }
    }

    pub fn show<W>(&self, w: &mut W)
        where W: Write
    {
        execute!(
            w,
            cursor::MoveTo(0, 0)
        ).unwrap();
        
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
        println!("得分: {}", self.score);
    }

    pub fn update_without_input(&mut self) {
        if self.bullet_y > -1 {
            self.bullet_y -= 1;
        }

        let rand_x = thread_rng().gen_range(0, self.width);

        if self.bullet_x == self.enemy_x && self.bullet_y == self.enemy_y {
            self.score += 1;
            self.enemy_y = -1;
            self.enemy_x = rand_x;
            self.bullet_y = -2;
        }

        // 超出显示区
        if self.enemy_y > self.high {
            self.enemy_y = -1;
            self.enemy_x = rand_x;
        }

        if self.speed < 5 {
            self.speed += 1;
        } else {
            self.enemy_y += 1;
            self.speed = 0;
        }

    }

    pub fn update_with_input(&mut self) -> Result<&str, &str> {
        if poll(Duration::from_millis(100)).unwrap() {
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
                    KeyCode::Esc => {  // 监听 esc 按下退出程序
                        return Ok("Exit");
                    },
                    _ => return Ok(""),
                }
            }
        }

        Ok("")
    }
}

pub fn main() {
    let mut stdout = io::stdout();
    
    execute!(
        stdout,
        cursor::Hide,   //隐藏光标
        terminal::EnterAlternateScreen
    ).unwrap();
    terminal::enable_raw_mode().unwrap();

    let mut app = App::new(20, 30);

    loop {
        app.show(&mut stdout);
        app.update_without_input();
        let code = app.update_with_input();
        match code {
            Ok(c) => {
                if c == "Exit" {
                    break;
                }
            },
            Err(_) => break,
        }
    }

    execute!(
        stdout,
        cursor::Show,
        terminal::LeaveAlternateScreen
    ).unwrap();
    terminal::disable_raw_mode().unwrap();
}