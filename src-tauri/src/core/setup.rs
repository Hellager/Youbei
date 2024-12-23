use tauri::{App, Manager};
use tauri_plugin_window_state::{WindowExt, StateFlags};
use crate::core::tray;

pub fn setup_handler(app: &mut App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let window = app.get_webview_window("main").unwrap();
    let _ = window.restore_state(StateFlags::all());
    log::info!("restore main window state");

    let _ = tray::create_system_tray(app.handle());

    Ok(())
}