// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Chat {
  message: String,
  time: String
}

#[tauri::command]
  fn send_chat_to_screen(app: tauri::AppHandle, payload: Vec<Chat>) {
    let chat_screen_window = app.get_window("chat-screen").unwrap();
    chat_screen_window.emit("recieve_chat", payload).unwrap();
}

// const SIZE: tauri::Size = tauri::Size::Physical(tauri::PhysicalSize{width: 256, height: 256});
fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    tauri::Builder::default()
        .setup(move |app|{
            // サブウィンドのマウス処理を無効化
            // use tauri::Manager;
            // let window = app.get_window("chat-screen").unwrap();
            // window.set_decorations(true).unwrap();
            // // window.set_always_on_top(true).unwrap();
            // // window.set_min_size(Some(SIZE)).unwrap();
            // // window.set_size(SIZE).unwrap();
            // window.center().unwrap();
            // let hwnd = window.hwnd().unwrap().0;
            // let _pre_val;
            // let hwnd = windows::Win32::Foundation::HWND(hwnd);
            // unsafe {
            //   use windows::Win32::UI::WindowsAndMessaging::*;
            //   let nindex = GWL_EXSTYLE;
            //   let style = WS_EX_APPWINDOW | WS_EX_COMPOSITED | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST;
            //   _pre_val = SetWindowLongA(hwnd, nindex, style.0 as i32);
            // };
            Ok(())
          })
        .invoke_handler(tauri::generate_handler![send_chat_to_screen,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
