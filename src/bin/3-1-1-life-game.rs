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

const HIGH: usize = 25;
const WIDTH: usize = 50;

struct App {
    pub cells: [[i32; WIDTH]; HIGH],
}

impl App {
    pub fn new() -> Self {
        let mut cells = [[0; WIDTH]; HIGH];

        for i in 0..HIGH {
            for j in 0..WIDTH {
                cells[i][j] = thread_rng().gen_range(0, 10) % 2;
            }
        }

        App {
            cells,
        }
    }

    pub fn show<W>(&self, w: &mut W) where W: Write {
        execute!(
            w,
            cursor::MoveTo(0, 0)
        ).unwrap();

        for i in 0..HIGH {
            for j in 0..WIDTH {
                if self.cells[i][j] == 1 {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    pub fn update_without_input(&mut self) -> Result<String, String> {
        Ok(String::from(""))
    }

    pub fn update_with_input(&mut self) -> Result<&str, &str> {
        if poll(Duration::from_millis(200)).unwrap() {
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