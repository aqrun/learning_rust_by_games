use std::process::Command;

/// 清屏函数
pub fn clear(){
    let msg = "Clear failed";
    let mut cmd = Command::new("clear");

    if cfg!(target_os = "windows") {
        cmd = Command::new("cmd");
        cmd.args(&["/C", "cls"]);
    }

    cmd.status().expect(msg);
}