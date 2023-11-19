// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::Command;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet() -> String {
    // 创建一个Command对象，指定要执行的程序名
    let mut cmd = Command::new("main.exe");
    // 调用spawn方法，启动外部程序
    let result = cmd.spawn();
    // 检查结果，如果成功，打印进程id，如果失败，打印错误信息
    return match result {
        Ok(child) => format!("Launched calculator with id {}", child.id()), // 使用format!宏
        Err(e) => format!("Failed to launch calculator: {}", e), // 使用format!宏
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
