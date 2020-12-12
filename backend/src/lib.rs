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
mod action;

use command::{Cmd, CommandError};

pub use command::{StoreState, ApplicationState, ActionCallback};

pub fn command_handler<T: ApplicationState + 'static>(
    webview: &mut Webview,
    arg: &str,
    state: &'static Lazy<Arc<StoreState<T>>>,
    commands: &'static Lazy<HashMap<String, Box<dyn Fn(T, serde_json::Value) -> tauri::Result<T> + Send + Sync + 'static>>>,
) -> Result<(), String> {
    match serde_json::from_str(arg) {
        Err(err) => {
            println!("{}", err);

            Err(err.to_string())
        },
        Ok(command) => {
            match command {
                Cmd::InitializeStore { callback, error } => tauri::execute_promise(webview, move || {
                    let state_data = state.data.lock().unwrap();
                    // *state_data = initialize_state();
                    println!("Init state: {}", *state_data);

                    Ok((*state_data).clone())
                }, callback, error),
                Cmd::Action { action, data, callback, error } => tauri::execute_promise(webview, move || {
                    // try to get function for action and execute it, otherwise return command error
                    match commands.get(&action) {
                        Some(ref func) => {
                            let mut state_data = state.data.lock().unwrap();
                            match func((*state_data).clone(), data) {
                                Ok(s) => {
                                    *state_data = s;
                                    println!("New state: {}", *state_data);

                                    Ok((*state_data).clone())
                                },
                                Err(err) => Err(err)
                            }
                        },
                        _ => {
                            println!("Unknown action: {}", action);

                            Err(CommandError::new(format!("Unknown action: {}", action)).into())
                        }
                    }
                }, callback, error)
            }

            Ok(())
        }
    }
}
