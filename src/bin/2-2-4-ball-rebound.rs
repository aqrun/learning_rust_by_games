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

/// 反弹小球
/// 
/// * 如果碰到档板则反弹 否则结束游戏
/// * 增加 ball_number 记录反弹次数
struct App {
    pub high: i32,
    pub width: i32,
    pub ball_x: i32,  // 球的位置
    pub ball_y: i32,
    pub ball_vx: i32,  // 球移动的相对距离
    pub ball_vy: i32,

    // 挡板中心坐标
    pub position_x: i32,
    pub position_y: i32,
    pub ridus: i32,  // 挡板半径大小
    pub left: i32,   // 挡板的左右位置
    pub right: i32,

    pub ball_number: i32,
}

impl App {
    pub fn new(high: i32, width: i32) -> App {
        let position_x = width / 2;
        let position_y = high / 2;
        let ridus = 5;

        App {
            high,
            width,
            ball_x: width / 2,
            ball_y: 0,
            ball_vx: 1,
            ball_vy: 1,

            ridus,
            position_x,
            position_y,
            left: position_x - ridus,
            right: position_x + ridus,

            ball_number: 0,
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
                if (i == self.ball_y) && (j == self.ball_x) {
                    print!("0"); // 输出小球
                } else if (i == (self.high - 1)) 
                    && (j >= self.left) 
                    && (j <= self.right)
                {
                    print!("*"); // 输出挡板
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

        println!("反弹小球数：{}", self.ball_number);
    }

    pub fn update_without_input(&mut self) -> Result<String, String> {
        if self.ball_y == self.high - 2 {
            if (self.ball_x >= self.left) && (self.ball_x <= self.right) {
                // 被挡板接住
                self.ball_number += 1;
                self.ball_vy = -self.ball_vy;
            } else {
                return Err(String::from("游戏失败！"));
            }
        }

        self.ball_x = self.ball_x + self.ball_vx;
        self.ball_y = self.ball_y + self.ball_vy;

        if (self.ball_x == 0) || (self.ball_x == self.width - 1) {
            self.ball_vx = -self.ball_vx;
        }
        if (self.ball_y == 0) || (self.ball_y == self.high - 1) {
            self.ball_vy = -self.ball_vy;
        }

        Ok(String::from(""))
    }

    pub fn update_with_input(&mut self) -> Result<&str, &str> {
        if poll(Duration::from_millis(100)).unwrap() {
            let evt = event::read().unwrap();
            if let Event::Key(KeyEvent{ code, .. }) = evt {
                match code {
                    KeyCode::Char('a') => {
                        self.position_x -= 1;
                        self.left = self.position_x - self.ridus;
                        self.right = self.position_x + self.ridus;
                    },
                    KeyCode::Char('d') => {
                        self.position_x += 1;
                        self.left = self.position_x - self.ridus;
                        self.right = self.position_x + self.ridus;
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

    let mut app = App::new(15, 20);
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
        println!("{}   反弹总数为：{}", e, app.ball_number);
    }
}