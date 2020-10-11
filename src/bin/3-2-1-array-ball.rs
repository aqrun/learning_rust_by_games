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

const HIGH: usize = 20;
const WIDTH: usize = 30;

/// 数组反弹球
struct App {
    pub ball_x: i32,
    pub ball_y: i32,
    pub ball_vx: i32,
    pub ball_vy: i32,
    pub canvas: Vec<Vec<i32>>,
}

impl App {
    pub fn new() -> Self {
        let ball_x = WIDTH / 2;
        let ball_y = 0;
        let mut canvas = vec![vec![0; WIDTH]; HIGH];
        canvas[ball_y][ball_x] = 1;

        App {
            ball_x: ball_x as i32,
            ball_y: ball_y as i32,
            ball_vx: 1,
            ball_vy: 1,
            canvas,
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
                }
            }
            println!("|");
        }
        for _ in 0..WIDTH {
            print!("-");
        }
        println!("+");
    }

    pub fn update_without_input(&mut self) -> Result<String, String> {
        self.canvas[self.ball_y as usize][self.ball_x as usize] = 0;
        self.ball_x += self.ball_vx;
        self.ball_y += self.ball_vy;

        if self.ball_x <= 0 || self.ball_x >= (WIDTH as i32 - 1) {
            self.ball_vx = -self.ball_vx;
        }
        if self.ball_y <= 0 || self.ball_y >= (HIGH as i32 - 1) {
            self.ball_vy = -self.ball_vy;
        }

        self.canvas[self.ball_y as usize][self.ball_x as usize] = 1;

        Ok(String::from(""))
    }

    pub fn update_with_input(&mut self) -> Result<&str, &str> {
        if poll(Duration::from_millis(50)).unwrap() {
            let evt = event::read().unwrap();
            if let Event::Key(KeyEvent{ code, .. }) = evt {
                match code {
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
}