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
  fn send_chat_to_screen(app: tauri::AppHandle, payload: Vec<Chat>) {
    let chat_screen_window = app.get_window("chat-screen").unwrap();
    chat_screen_window.emit("recieve_chat", payload).unwrap();
}

#[tauri::command]
async fn create_child_window(_app: tauri::AppHandle, meet: Meet) {
  create_meet_window_async(_app.clone(), meet.id).await;
  create_chat_screen_window_async(_app.clone()).await;
}

#[tauri::command]
async fn create_meet_window_async(_app: AppHandle, room_id: String) {
  let _exists_meet_window = _app.get_window("meet").is_some();
  if _exists_meet_window { return  (); }

  let _chat_window = _app.get_window("chat").unwrap();
  let _position = calc_meet_position(_chat_window.clone());
  let _size = calc_meet_size(_chat_window.clone());

  let room_url = format!("https://meet.google.com/{room_id}");
  let child = window::WindowBuilder::new(
    &_app, 
    "meet", 
    tauri::WindowUrl::External(room_url.parse().unwrap()))
    .title("meet")
    .decorations(true)
    .resizable(true)
    .position(
      _position.x,
      _position.y,
    )
    .inner_size(
      _size.x,
      _size.y
    );

  child.build().expect("faild to build window");
  return ();
}

fn calc_meet_size(_chat_window: tauri::Window) -> tauri::LogicalPosition<f64> {
  let scale_factor = _chat_window.scale_factor().unwrap();

  let _chat_window_phyiscal_size = _chat_window.outer_size().unwrap();
  let _chat_window_size = _chat_window_phyiscal_size.to_logical::<i32>(scale_factor);

  let _header_height = 40.0;
  let _meet_width = 1200.0;
  let _meet_height = _chat_window_size.height as f64 - _header_height;

  let _size = tauri::LogicalPosition::new(_meet_width, _meet_height);
  return  _size;
}

fn calc_meet_position(_chat_window: tauri::Window) -> tauri::LogicalPosition<f64> {
  let scale_factor = _chat_window.scale_factor().unwrap();

  let _chat_window_phyiscal_pos = _chat_window.outer_position().unwrap();
  let _chat_window_pos = _chat_window_phyiscal_pos.to_logical::<i32>(scale_factor);

  let _meet_width = 1200.0;

  let _meet_pos_x = _chat_window_pos.x as f64 - _meet_width - 5.0;
  let _meet_pos_y = _chat_window_pos.y as f64;

  let _size = tauri::LogicalPosition::new(_meet_pos_x, _meet_pos_y);
  return  _size;
}

#[tauri::command]
async fn create_chat_screen_window_async(_app: tauri::AppHandle) {
  let _exists_chat_screen_window = _app.get_window("chat-screen").is_some();
  if _exists_chat_screen_window { return (); }

  let _meet_window = _app.get_window("meet").unwrap();
  let _chat_screen_position = calc_chat_screen_position(_meet_window.clone());
  let _chat_screen_size = calc_chat_screen_size(_meet_window.clone());

  let screen_url = "http://localhost:5173/chatscreen";
  let child = window::WindowBuilder::new(
    &_app, 
    "chat-screen", 
    tauri::WindowUrl::External(screen_url.parse().unwrap()))
    .title("chat-screen")
    .always_on_top(false)
    .decorations(false)
    .resizable(false)
    .position(
      _chat_screen_position.x,
      _chat_screen_position.y,
    )
    .inner_size(
      _chat_screen_size.x,
      _chat_screen_size.y
    ).transparent(true);

  let window = child.build().expect("faild to build window");
  let hwnd = window.hwnd().unwrap().0;
  let hwnd = windows::Win32::Foundation::HWND(hwnd);
  unsafe {
    use windows::Win32::UI::WindowsAndMessaging::*;
    let nindex = GWL_EXSTYLE;
    let style = WS_EX_APPWINDOW | WS_EX_COMPOSITED | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST;
    let _pre_val = SetWindowLongA(hwnd, nindex, style.0 as i32);
  };

  return ();
}

fn calc_chat_screen_size(_meet_window: tauri::Window) -> tauri::LogicalPosition<f64> {
  let _scale_factor = _meet_window.scale_factor().unwrap();
  let _meet_window_phyiscal_size = _meet_window.outer_size().unwrap();
  let _meet_window_size = _meet_window_phyiscal_size.to_logical::<i32>(_scale_factor);

  let _header_height = 40.0;
  let _offset_width = -15.0;
  let _chat_screen_width = _meet_window_size.width as f64 + _offset_width;
  let _chat_screen_height = _meet_window_size.height as f64 - _header_height;

  let _size = tauri::LogicalPosition::new(_chat_screen_width, _chat_screen_height);
  return _size;
}

fn calc_chat_screen_position(_meet_window: tauri::Window) -> tauri::LogicalPosition<f64> {
  let _scale_factor = _meet_window.scale_factor().unwrap();
  let _meet_window_phyiscal_pos = _meet_window.outer_position().unwrap();
  let _meet_window_pos = _meet_window_phyiscal_pos.to_logical::<i32>(_scale_factor);

  let _header_height = 40.0;
  let _decorations_offset_x = 8.0;
  let _decorations_offset_y = -9.0;
  let _chat_screen_pos_x = _meet_window_pos.x as f64 + _decorations_offset_x;
  let _chat_screen_pos_y = _meet_window_pos.y as f64 + _header_height + _decorations_offset_y;  

  let _position = tauri::LogicalPosition::new(_chat_screen_pos_x, _chat_screen_pos_y);
  return  _position;
}

// const SIZE: tauri::Size = tauri::Size::Physical(tauri::PhysicalSize{width: 256, height: 256});
fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    tauri::Builder::default()
        .setup(move |_app|{
            Ok(())
          })
          .on_window_event(move |global_event| match global_event.event() {
            tauri::WindowEvent::Moved(_position) => {
              let is_meet_window = global_event.window().label() == "meet";
              if !is_meet_window { return; }

              // meetウィンドにchat_screenを追従させる
              let _meet_window = global_event.window();
              let _screen_position = calc_chat_screen_position(_meet_window.clone());
              let _screen_window = global_event.window().app_handle().get_window("chat-screen").unwrap();
              let _ = _screen_window.set_position(_screen_position);
            }
            tauri::WindowEvent::Resized(_size) => { 
              let is_meet_window = global_event.window().label() == "meet";
              if !is_meet_window { return; }

              // meetウィンドにchat_screenを追従させる
              let _meet_window = global_event.window();
              let _screen_size = calc_chat_screen_size(_meet_window.clone());
              let _screen_window = global_event.window().app_handle().get_window("chat-screen").unwrap();
              let _ = _screen_window.set_size(tauri::Size::Physical(tauri::PhysicalSize { width: _screen_size.x as u32, height: _screen_size.y as u32 }));
             }
             tauri::WindowEvent::Focused(_focused) => {
              let _screen_window = global_event.window().app_handle().get_window("chat-screen");
              if _screen_window == None { return; }

              let _is_screen_window = global_event.window().label() == "chat-screen";
              if _is_screen_window { return; }
              
              let _window = _screen_window.unwrap();
              let _ = _window.set_always_on_top(*_focused); 

              // 毎回設定しないとクリックの透過ができない
              let hwnd = _window.hwnd().unwrap().0;
              let hwnd = windows::Win32::Foundation::HWND(hwnd);
              unsafe {
                use windows::Win32::UI::WindowsAndMessaging::*;
                let nindex = GWL_EXSTYLE;
                let style = WS_EX_APPWINDOW | WS_EX_COMPOSITED | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST;
                let _pre_val = SetWindowLongA(hwnd, nindex, style.0 as i32);
              };
             }
            tauri::WindowEvent::Destroyed => {
               let is_chat_window = global_event.window().label() == "chat";
               if is_chat_window { std::process::exit(0x0); }
            }
            _ => (),
          })
        .invoke_handler(tauri::generate_handler![create_child_window, send_chat_to_screen,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
