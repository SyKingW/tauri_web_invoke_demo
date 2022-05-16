#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{thread::{self, sleep}, time::Duration};

use serde::Deserialize;
use tauri::{State, Manager};

fn main() {
    let mut app = tauri::Builder::default();
    
    // app = app.manage(WebString::default()) // invoke 数据不对时，默认的数据
    // app = app.manage(WebU8::default())    
    // 注册函数给 web 调用
    app = app.invoke_handler(tauri::generate_handler![
        command_1, command_2, command_3,
        // etc...
    ]);

    app = app.on_page_load(|window, p| {
        println!("on_page_load: {:?}", p);
        // 监听 web 页面加载，并且去持有 windows，发送 rust 消息给 web
        thread::spawn(move || {
            loop {
                match window.emit("rust-event", "rust_emit_msg 123123") {
                    Ok(_) => {},
                    Err(e) => {
                        println!("error: {}", e);
                    },
                }
                sleep(Duration::from_millis(1000));
            }
        });
    });

    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Default, Deserialize)]
struct WebString {
    v: String,
}

#[derive(Debug, Default, Deserialize)]
struct WebU8 {
    v: u8,
}

/// 接收到 web 命令, 并且返回值给 web
#[tauri::command]
fn command_1() -> String {
    println!("command hello world");
    return "hello world".to_string();
}

/// 接收到 web 带参数命令
#[tauri::command]
fn command_2(p: WebString) {
    println!("command_2: {:?}", p);
}

#[tauri::command]
fn command_3(p: WebU8) {
    println!("command_3: {:?}", p);
}
