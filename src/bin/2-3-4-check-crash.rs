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

/// 判断碰撞
struct App {
    pub high: i32,
    pub width: i32,
    pub bird_x: i32,
    pub bird_y: i32,
    pub bar1_x: i32,
    pub bar1_y_top: i32,
    pub bar1_y_bottom: i32,
    pub score: i32,
}

impl App {
    pub fn new(high: i32, width: i32) -> Self {
        App {
            high,
            width,
            bird_y: 0,
            bird_x: width / 3,

            bar1_x: width / 2,
            bar1_y_top: high / 2,
            bar1_y_bottom: high / 3,

            score: 0,
        }
    }

    pub fn show<W>(&self, w: &mut W) where W: Write {
        execute!(
            w,
            cursor::MoveTo(0, 0)
        ).unwrap();

        for i in 0..(self.high + 1) {
            for j in 0..(self.width + 1) {
                if (i == self.bird_y) && (j == self.bird_x) {
                    print!("@");
                } else if (j == self.bar1_x)
                    && 
                    (
                        i < self.bar1_y_bottom
                        || 
                        i > self.bar1_y_top
                    )
                    &&
                    i < self.high 
                {
                    print!("*");
                } else if i == self.high {
                    print!("-");
                } else if j == self.width {
                    print!("|")
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
        println!("得分: {}", self.score);
    }

    pub fn update_without_input(&mut self) -> Result<String, String> {
        self.bird_y += 1;
        self.bar1_x -= 1;

        if self.bird_y >= self.high {
            return Err(String::from("Game over"));
        }

        if self.bird_x == self.bar1_x {
            if self.bird_y >= self.bar1_y_bottom
                && self.bird_y <= self.bar1_y_top 
            {
                self.score += 1;
            } else {
                return Err(String::from("Game over"));
            }
        }

        Ok(String::from(""))
    }

    pub fn update_with_input(&mut self) -> Result<&str, &str> {
        if poll(Duration::from_millis(200)).unwrap() {
            let evt = event::read().unwrap();
            if let Event::Key(KeyEvent{ code, .. }) = evt {
                match code {
                    KeyCode::Char(' ') => {
                        self.bird_y -= 2;
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

    let mut app = App::new(20, 60);
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

    if let Err(e) = run_res {
        println!("{}", e);
        println!("得分: {}", app.score);
    }
}