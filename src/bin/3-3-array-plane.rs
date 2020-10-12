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

const HIGH: usize = 25;
const WIDTH: usize = 50;

/// 空战游戏
struct App {
    pub position_x: i32,
    pub position_y: i32,
    pub canvas: Vec<Vec<i32>>,

    pub score: i32,
}

impl App {
    pub fn new() -> Self {
        let position_x = WIDTH / 2;
        let position_y = HIGH / 2;
        let mut canvas = vec![vec![0; WIDTH]; HIGH];
        canvas[position_y][position_x] = 1;

        App {
            canvas,
            position_x: position_x as i32,
            position_y: position_y as i32,

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
                   print!("*");
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
        thread::sleep(Duration::from_millis(150));
        Ok(String::from(""))
    }

    pub fn update_with_input(&mut self) -> Result<&str, &str> {
        if poll(Duration::from_millis(50)).unwrap() {
            let evt = event::read().unwrap();
            if let Event::Key(KeyEvent{ code, .. }) = evt {
                match code {
                    KeyCode::Char('a') => {
                        self.set_canvas(0);
                        self.position_x -= 1;
                        self.set_canvas(1);
                    },
                    KeyCode::Char('d') => {
                        self.set_canvas(0);
                        self.position_x += 1;
                        self.set_canvas(1);
                    },
                    KeyCode::Char('w') => {
                        self.set_canvas(0);
                        self.position_y -= 1;
                        self.set_canvas(1);
                    },
                    KeyCode::Char('s') => {
                        self.set_canvas(0);
                        self.position_y += 1;
                        self.set_canvas(1);
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

    pub fn set_canvas(&mut self, val: i32) {
        self.canvas[self.position_y as usize][self.position_x as usize] = val;
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