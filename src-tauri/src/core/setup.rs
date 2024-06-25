//! # Setup
//! 
//! `setup` module contains functions about setup tauri app itself and its' commands.
use tauri::{App, Manager};

/// Setup tauri app.
/// 
/// # Arguments
/// 
/// * `app` - An app instance
/// 
/// # Examples
/// 
/// ```
/// use core::setup::setup_handler;
/// 
/// fn main() {
///     tauri::Builder::default()
///         .setup(crate::core::setup::setup_handler)
///         .run(tauri::generate_context!())
///         .expect("error while running tauri application");
/// }
/// ```
pub fn setup_handler(app: &mut App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    use super::state::{ MissionHandler, HandlerStatus, MissionHandlerState };
    use super::window;
    use crate::config::AppConfig;
    use tokio::sync::Mutex;
    use log::error;

    if let Some(main_window) = app.get_window("main") {
        // add window shadows to app, not available on linux now
        #[cfg(not(target_os = "linux"))]
        window::init_window_shadow(&main_window, true);
    } else {
        error!("failed to init window shadow");
    }

    let state = MissionHandlerState(Mutex::new(MissionHandler {
        is_set: false,
        status: HandlerStatus::default(),
        config: AppConfig::default(),

        app_handler: Some(app.handle().clone()),
        log_handler: None,
        db_handler: None,
    }));

    app.manage(state);

    Ok(())
}

/// Setup tauri commands.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use core::setup::setup_command;
/// 
/// fn main() {
///     tauri::Builder::default()
///         .invoke_handler(crate::core::setup::setup_command())
///         .run(tauri::generate_context!())
///         .expect("error while running tauri application");
/// }
/// ```
pub fn setup_command() -> Box<dyn Fn(tauri::Invoke<tauri::Wry>) + Send + Sync> {
    use super::cmd::*;

    Box::new(tauri::generate_handler![
        init_app,
        shutdown_app
    ])
}