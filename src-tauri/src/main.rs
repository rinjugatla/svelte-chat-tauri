// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, window, AppHandle};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Chat {
  id: String,
  message: String,
  time: String
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Meet {
  id: String
}

#[tauri::command]
async fn create_meet_window(app: tauri::AppHandle, meet: Meet) {
  create_meet_window_async(app, meet.id).await;
}

#[tauri::command]
  fn send_chat_to_screen(app: tauri::AppHandle, payload: Vec<Chat>) {
    let chat_screen_window = app.get_window("chat-screen").unwrap();
    chat_screen_window.emit("recieve_chat", payload).unwrap();
}

#[tauri::command]
async fn create_meet_window_async(_app: AppHandle, room_id: String) {
  let _exists_meet_window = _app.get_window("meet").is_some();
  if _exists_meet_window { return  (); }

  let main = _app.get_window("chat").unwrap();
  let scale_factor = main.scale_factor().unwrap();

  let main_phyiscal_pos = main.outer_position().unwrap();
  let main_pos = main_phyiscal_pos.to_logical::<i32>(scale_factor);

  let main_phyiscal_size = main.outer_size().unwrap();
  let main_size = main_phyiscal_size.to_logical::<i32>(scale_factor);

  let _meet_width = 1200.0;
  let _meet_height = main_size.height as f64 - 40.0;
  let _meet_pos_x = main_pos.x as f64 - _meet_width - 5.0;
  let _meet_pos_y = main_pos.y as f64;

  let room_url = format!("https://meet.google.com/{room_id}");
  let child = window::WindowBuilder::new(
    &_app, 
    "meet", 
    tauri::WindowUrl::External(room_url.parse().unwrap()))
    .title("meet")
    .decorations(true)
    .resizable(false)
    .position(
      _meet_pos_x,
      _meet_pos_y,
    )
    .inner_size(
      _meet_width,
      _meet_height
    );

  child.build().expect("faild to build window");
  return ();
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
        .invoke_handler(tauri::generate_handler![create_meet_window, send_chat_to_screen,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
