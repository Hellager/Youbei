//! # Cmd
//! 
//! `Cmd` module contains all commands that interacts with frontend.

use tauri::{ command, AppHandle, Manager, State, Window };
use serde::{Serialize, Deserialize};
use log::error;
use crate::core::state::{ HandlerStatus, MissionHandlerState };

/// Struct for command response
#[derive(Clone, Serialize, Deserialize)]
pub struct Response<T> {
    /// Response status code
    /// More like HTTP response status codes
    pub code: i32,

    /// Response data
    /// Should be able to be serialize and deserialize
    pub data: T,

    /// Additional message
    pub msg: String,
}

impl<T> Response<T> {
    /// Build a success response with genertic type.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use core::cmd::Response;
    /// 
    /// let response = Response::success(true);
    /// 
    /// println!("success response: {:?}", response);
    /// ```
    pub fn success(value: T) -> Response<T> {
        Response {
            code: 200,
            data: value,
            msg: "".to_string(),
        }
    }

    /// Build a success response with error message.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use core::cmd::Response;
    /// 
    /// let response = Response::<bool>::error(404, format!("{:?}", "not found"));
    /// 
    /// println!("error response: {:?}", response);
    /// ```
    pub fn error(err_code: i32, err_msg: String) -> Response<bool> {
        Response {
            code: err_code,
            data: false,
            msg: err_msg
        }
    }
}

/// Command for init app.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```js
/// import { invoke } from '@tauri-apps/api/tauri'
/// 
/// await invoke('init_app')
///     .then(res => {
///         console.log("init app success")
///         console.log(res.data)
///     })
///     .catch(err => {
///         console.error(err)
///     })
/// ```
#[command]
pub async fn init_app(window: Window, state: State<'_, MissionHandlerState>) -> Result<Response<HandlerStatus>, Response<bool>> {    
    let mut guard = state.0.lock().await;
    if !guard.is_set {
        if let Err(error) = guard.initialize() {
            error!("Failed to initialize state, errMsg: {:?}", error);
            return Err(Response::<bool>::error(500, format!("{:?}", error)));
        }     

        // close splashscreen
        if let Some(splashscreen) = window.get_window("splashscreen") {
            if let Err(error) = splashscreen.close() {
                error!("failed to init app, errMsg: {:?}", error);
                return Err(Response::<bool>::error(500, format!("{:?}", error)));
            }
        } else {
            error!("missing splashsceen window");
            return Err(Response::<bool>::error(404, format!("missing splashsceen window")));
        }        
    }

    // Show main window
    if let Some(main_window) = window.get_window("main") {
        if let Err(error) = main_window.show() {
            error!("failed to show main window, errMsg: {:?}", error);
            return Err(Response::<bool>::error(500, format!("{:?}", error)));
        }
    } else {
        error!("missing main window");
        return Err(Response::<bool>::error(404, format!("missing main window")));
    }
    
    let status = guard.status.clone();
    
    Ok(Response::success(status))
}

/// Command for shutdown app.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```js
/// import { invoke } from '@tauri-apps/api/tauri'
/// 
/// await invoke('shutdown_app')
/// ```
#[command]
pub async fn shutdown_app(app: AppHandle, state: State<'_, MissionHandlerState>) -> Result<Response<bool>, Response<bool>> {
    let mut guard = state.0.lock().await;

    match guard.shutdown() {
        Ok(()) => {
            app.exit(0);

            return Ok(Response::success(true));
        },
        Err(error) => {
            error!("Failed to shutdown, errMsg: {:?}", error);
            return Err(Response::<bool>::error(500, format!("{:?}", error)));
        }
    }
}