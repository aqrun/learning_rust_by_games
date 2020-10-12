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
const ENEMY_NUM: i32 = 5;

/// 空战游戏
/// 击中敌机
struct App {
    pub position_x: i32,
    pub position_y: i32,
    pub canvas: Vec<Vec<i32>>,
    pub enemy_x: Vec<usize>,
    pub enemy_y: Vec<usize>,

    pub score: i32,
    pub speed: i32,
}

impl App {
    pub fn new() -> Self {
        let position_x = WIDTH / 2;
        let position_y = HIGH / 2;

        let mut enemy_x = vec![0_usize; ENEMY_NUM as usize];
        let mut enemy_y = vec![0_usize; ENEMY_NUM as usize];

        let mut canvas = vec![vec![0; WIDTH]; HIGH];
        canvas[position_y][position_x] = 1;
        
        for i in 0..ENEMY_NUM {
            enemy_x[i as usize] = thread_rng().gen_range(0, WIDTH);
            enemy_y[i as usize] = thread_rng().gen_range(0, 2_usize);
            canvas[enemy_y[i as usize]][enemy_x[i as usize]] = 3;
        }

        App {
            canvas,
            position_x: position_x as i32,
            position_y: position_y as i32,
            enemy_x,
            enemy_y,

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
        for i in 0..HIGH {
            for j in 0..WIDTH {
                if self.canvas[i][j] == 2 {
                    for k in 0..(ENEMY_NUM as usize) {
                        // 子弹击中敌机
                        if (i == self.enemy_y[k]) && (j == self.enemy_x[k]) {
                            self.score += 1;
                            self.set_usize_val(self.enemy_x[k], self.enemy_y[k], 0);
                            self.enemy_y[k] = thread_rng().gen_range(0, 2_usize);
                            self.enemy_x[k] = thread_rng().gen_range(0, WIDTH);
                            self.set_usize_val(self.enemy_x[k], self.enemy_y[k], 0);
                            self.set_usize_val(j, i, 0);
                        }
                    }

                    // 子弹向上移动
                    self.set_usize_val(j, i, 0);
                    if i > 0 {
                        self.set_usize_val(j, i - 1, 2);
                    }
                }
            }
        }

        if self.speed < 10 {
            self.speed += 1;
        }

        for kk in 0..ENEMY_NUM {
            let k = kk as usize;
            // 检测敌机撞到我机
            if (self.position_x == self.enemy_x[k] as i32) 
                && (self.position_y == self.enemy_y[k] as i32
            ) {
                return Err(String::from("game over"));
            }

            // 敌机跑出显示屏
            if self.enemy_y[k] > HIGH {
                self.set_usize_val(self.enemy_x[k], self.enemy_y[k], 0);
                self.enemy_y[k] = thread_rng().gen_range(0, 2_usize);
                self.enemy_x[k] = thread_rng().gen_range(0, WIDTH);
                self.set_usize_val(self.enemy_x[k], self.enemy_y[k], 3);
                self.score -= 1;
            }

            if self.speed >= 10 {
                for m in 0..(ENEMY_NUM as usize) {
                    self.set_usize_val(self.enemy_x[m], self.enemy_y[m], 0);
                    self.enemy_y[m] += 1;
                    self.speed = 0;
                    self.set_usize_val(self.enemy_x[m], self.enemy_y[m], 3);
                }
            }
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
        let mut x = pos_x;
        let mut y = pos_y;
        let w = WIDTH as i32;
        let h = HIGH as i32;

        if x >= w - 1 {
            x = w - 1;
        }

        if y >= h - 1 {
            y = h - 2;
        }
        self.canvas[y as usize][x as usize] = val;
    }

    pub fn set_usize_val(&mut self, pos_x: usize, pos_y: usize, val: i32) {
        self.canvas[pos_y][pos_x] = val;
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
