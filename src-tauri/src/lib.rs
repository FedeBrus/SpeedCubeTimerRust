mod timer;

use std::sync::Mutex;
use tauri::State;
use timer::Timer;

struct AppState {
    timer: Mutex<Timer>,
}

#[tauri::command]
fn start_timer(state: State<AppState>) {
    let mut timer = state.timer.lock().unwrap();
    timer.start();
}

#[tauri::command]
fn stop_timer(state: State<AppState>) -> Option<u128> {
    let mut timer = state.timer.lock().unwrap();
    timer.stop()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(AppState {
        timer: Mutex::new(Timer::new()),
    })
    .invoke_handler(tauri::generate_handler![
        start_timer,
        stop_timer,
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
