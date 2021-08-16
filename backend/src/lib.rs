/*
 * File: lib.rs
 * Author: MarkAtk
 * Date: 09.12.20
 *
 * MIT License
 *
 * Copyright (c) 2020 MarkAtk
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
use tauri::Webview;

mod command;

use command::{Cmd, CommandError};

pub use command::{StoreState, ApplicationState, ActionCallback};

pub const INIT_FUNC: &str = "initialize";

pub fn command_handler<T: ApplicationState + 'static>(
    webview: &mut Webview,
    arg: &str,
    state: &'static Lazy<Arc<StoreState<T>>>,
    commands: &'static Lazy<HashMap<String, Box<dyn Fn(&mut T, serde_json::Value) -> tauri::Result<()> + Send + Sync + 'static>>>,
) -> Result<(), String> {
    // parse command from frontend argument
    let command = serde_json::from_str(arg);
    if let Err(err) = command {
        println!("{}", err);

        return Err(err.to_string());
    }

    // handle parsed command
    // TODO: Add unhandled command match
    match command.unwrap() {
        Cmd::InitializeStore { callback, error } => tauri::execute_promise(
            webview,
            move || initialize_store_handler(state, commands),
            callback,
            error
        ),
        Cmd::Action { action, data, callback, error } => tauri::execute_promise(
            webview,
            move || action_handler(state, commands, action, data),
            callback,
            error
        )
    }

    Ok(())
}

pub fn update_state<T: ApplicationState>(webview: &mut tauri::WebviewMut, state: T) -> tauri::Result<()> {
    tauri::event::emit(webview, String::from("state-update"), Some(state))
}

fn initialize_store_handler<T: ApplicationState + 'static>(
    state: &'static Lazy<Arc<StoreState<T>>>,
    commands: &'static Lazy<HashMap<String, Box<dyn Fn(&mut T, serde_json::Value) -> tauri::Result<()> + Send + Sync + 'static>>>
) -> tauri::Result<()> {
    // (should be) called once when the application starts to set initial store values
    let mut state_data = state.data.lock().unwrap();

    match commands.get(INIT_FUNC) {
        Some(ref func) => {
            // execute the given callback in the backend for the initialize store event
            match func(&mut (*state_data), serde_json::Value::Null) {
                Ok(_) => {
                    println!("Init state: {}", *state_data);

                    Ok(())
                },
                Err(err) => Err(err)
            }
        },
        _ => Ok(())
    }
}

fn action_handler<T: ApplicationState + 'static>(
    state: &'static Lazy<Arc<StoreState<T>>>,
    commands: &'static Lazy<HashMap<String, Box<dyn Fn(&mut T, serde_json::Value) -> tauri::Result<()> + Send + Sync + 'static>>>,
    action: String,
    data: serde_json::Value
) -> tauri::Result<()> {
    // try to get function for action and execute it, otherwise return command error
    match commands.get(&action) {
        Some(ref func) => {
            let mut state_data = state.data.lock().unwrap();

            // execute the given callback in the backend for the action
            match func(&mut (*state_data), data) {
                Ok(_) => {
                    println!("New state: {}", *state_data);

                    Ok(())
                },
                Err(err) => Err(err)
            }
        },
        _ => {
            println!("Unknown action: {}", action);

            Err(CommandError::new(format!("Unknown action: {}", action)).into())
        }
    }
}
