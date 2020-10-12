use std::io::{self, Write};
use std::time::Duration;
use std::thread;
use crossterm::{
    cursor,
    event::{
        self, poll, Event, KeyCode, KeyEvent
    },
    execute,
    terminal,
};
use rand::prelude::*;

const HIGH: usize = 20;
const WIDTH: usize = 30;

/// 数组反弹球 消砖块
/// 0 是空白
/// 1 是小球
/// 2 是挡板
/// 3 是砖块
struct App {
    pub ball_x: i32,
    pub ball_y: i32,
    pub ball_vx: i32,
    pub ball_vy: i32,
    pub canvas: Vec<Vec<i32>>,

    pub position_x: i32,    // 挡板的中心坐标
    pub position_y: i32,
    pub ridus: i32,         // 挡板半径
    pub left: i32,          // 挡板左右位置
    pub right: i32,

    pub score: i32,
}

impl App {
    pub fn new() -> Self {
        let ball_x = WIDTH / 2;
        let ball_y = HIGH / 2;
        let mut canvas = vec![vec![0; WIDTH]; HIGH];
        canvas[ball_y][ball_x] = 1;

        let ridus = 5;
        let position_x = WIDTH / 2;
        let position_y = HIGH - 1;
        let left = position_x - ridus;
        let right = position_x + ridus;

        for i in left..right {
            canvas[position_y][i] = 2;
        }

        for i in 0..3 {
            for j in 0..WIDTH {
                canvas[i][j] = 3;
            }
        }

        App {
            ball_x: ball_x as i32,
            ball_y: ball_y as i32,
            ball_vx: 1,
            ball_vy: 1,
            canvas,

            ridus: ridus as i32,
            position_x: position_x as i32,
            position_y: position_y as i32,
            left: left as i32,
            right: right as i32,

            score: 0,
        }
    }

    pub fn show<W>(&self, w: &mut W) where W: Write {
        execute!(
            w,
            cursor::MoveTo(0, 0)
        ).unwrap();

        for i in 0..HIGH {
            for j in 0..WIDTH {
                if self.canvas[i][j] == 0 {
                    print!(" ");
                } else if self.canvas[i][j] == 1 {
                   print!("0");
                } else if self.canvas[i][j] == 2 {
                    print!("*");
                } else if self.canvas[i][j] == 3 {
                    print!("#");
                }
            }
            println!("|");
        }
        for _ in 0..WIDTH {
            print!("-");
        }
        println!("+");

        println!("得分：{}", self.score);
    }

    pub fn update_without_input(&mut self) -> Result<String, String> {
        if self.ball_y == HIGH as i32 - 2 {
            if (self.ball_x < self.left) || (self.ball_x > self.right) {
                return Err(String::from("Game over"));
            }
        }

        self.canvas[self.ball_y as usize][self.ball_x as usize] = 0;
        self.ball_x += self.ball_vx;
        self.ball_y += self.ball_vy;

        if self.ball_x <= 0 || self.ball_x >= (WIDTH as i32 - 1) {
            self.ball_vx = -self.ball_vx;
        }
        if self.ball_y <= 0 || self.ball_y >= (HIGH as i32 - 2) {
            self.ball_vy = -self.ball_vy;
        }

        self.canvas[self.ball_y as usize][self.ball_x as usize] = 1;

        let mut new_ball_y = self.ball_y - 1;

        if new_ball_y <= 0 {
            new_ball_y = 0;
        }

        if self.canvas[new_ball_y as usize][self.ball_x as usize] == 3 {
            self.ball_vy = -self.ball_vy;
            self.canvas[new_ball_y as usize][self.ball_x as usize] = 0;

            self.score += 1;
        }

        thread::sleep(Duration::from_millis(150));
        Ok(String::from(""))
    }

    pub fn update_with_input(&mut self) -> Result<&str, &str> {
        if poll(Duration::from_millis(50)).unwrap() {
            let evt = event::read().unwrap();
            if let Event::Key(KeyEvent{ code, .. }) = evt {
                match code {
                    KeyCode::Char('a') if self.left > 0 => {
                        self.canvas[self.position_y as usize][self.right as usize] = 0;
                        self.position_x -= 1;
                        self.left = self.position_x - self.ridus;
                        self.right = self.position_x + self.ridus;
                        self.canvas[self.position_y as usize][self.left as usize] = 2;
                    },
                    KeyCode::Char('d') if self.right < (WIDTH as i32) - 1 => {
                        self.canvas[self.position_y as usize][self.left as usize] = 0;
                        self.position_x += 1;
                        self.left = self.position_x - self.ridus;
                        self.right = self.position_x + self.ridus;
                        self.canvas[self.position_y as usize][self.right as usize] = 2;
                    },
                    KeyCode::Esc => {
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
        cursor::Hide,
        terminal::EnterAlternateScreen
    ).unwrap();
    terminal::enable_raw_mode().unwrap();

    let mut app = App::new();
    let mut run_res: Result<String, String>;
    
    loop {
        app.show(&mut stdout);
        run_res = app.update_without_input();

        if let Err(_) = run_res {
            break;
        }

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

    if let Err(_) = run_res {
        println!("得分：{}", app.score);
    }
}