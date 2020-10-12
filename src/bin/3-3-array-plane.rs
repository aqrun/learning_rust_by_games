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
/// 击中敌机
struct App {
    pub position_x: i32,
    pub position_y: i32,
    pub canvas: Vec<Vec<i32>>,
    pub enemy_x: i32,
    pub enemy_y: i32,

    pub score: i32,

    pub speed: i32,
}

impl App {
    pub fn new() -> Self {
        let position_x = WIDTH / 2;
        let position_y = HIGH / 2;
        let enemy_x = position_x;
        let enemy_y = 0;

        let mut canvas = vec![vec![0; WIDTH]; HIGH];
        canvas[position_y][position_x] = 1;
        canvas[enemy_y][enemy_x] = 3;

        App {
            canvas,
            position_x: position_x as i32,
            position_y: position_y as i32,
            enemy_x: enemy_x as i32,
            enemy_y: enemy_y as i32,

            score: 0,
            speed: 0,
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
                } else if self.canvas[i][j] == 2 {
                    print!("|");
                } else if self.canvas[i][j] == 3 {
                    print!("@");
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
        for i in 0..(HIGH as i32) {
            for j in 0..(WIDTH as i32) {
                if self.canvas[i as usize][j as usize] == 2 {
                    // 子弹击中敌机
                    if (i == self.enemy_y) && (j == self.enemy_x) {
                        self.score += 1;
                        self.set_pos_val(self.enemy_x, self.enemy_y, 0);
                        self.enemy_y = 0;
                        self.enemy_x = thread_rng().gen_range(0, WIDTH as i32);
                        self.set_pos_val(self.enemy_x, self.enemy_y, 0);
                        self.set_pos_val(j, i, 0);
                    }

                    // 子弹向上移动
                    self.set_pos_val(j, i, 0);
                    if i > 0 {
                        self.set_pos_val(j, i - 1, 2);
                    }
                }
            }
        }

        // 检测敌机撞到我机
        if (self.position_x == self.enemy_x) && (self.position_y == self.enemy_y) {
            return Err(String::from("game over"));
        }

        // 敌机跑出显示屏
        if self.enemy_y > HIGH as i32 {
            self.set_pos_val(self.enemy_x, self.enemy_y, 0);
            self.enemy_y = 0;
            self.enemy_x = thread_rng().gen_range(0, WIDTH as i32);
            self.set_pos_val(self.enemy_x, self.enemy_y, 3);
            self.score -= 1;
        }

        if self.speed < 10 {
            self.speed += 1;
        }

        if self.speed >= 10 {
            self.set_pos_val(self.enemy_x, self.enemy_y, 0);
            self.enemy_y += 1;
            self.speed = 0;
            self.set_pos_val(self.enemy_x, self.enemy_y, 3);
        }

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
                    KeyCode::Char(' ') => {
                        self.set_pos_val(self.position_x, self.position_y - 1, 2);
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
        let mut x = self.position_x;
        let mut y = self.position_y;
        let w = WIDTH as i32;
        let h = HIGH as i32;

        if x >= w - 1 {
            x = w - 1;
        }

        if y >= h - 1 {
            y = h - 1;
        }
        self.canvas[y as usize][x as usize] = val;
    }

    pub fn set_pos_val(&mut self, pos_x: i32, pos_y: i32, val: i32) {
        self.canvas[pos_y as usize][pos_x as usize] = val;
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

    println!("得分：{}", app.score);
}
