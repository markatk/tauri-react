/*
 * File: main.rs
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

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::thread;
use std::time::Duration;
use tauri_react::command_handler;

mod state;
mod commands;

fn main() {
    tauri::AppBuilder::new()
        .setup(|webview, _source| {
            let mut webview = webview.as_mut();

            thread::spawn(move || {
                loop {
                    let time = std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    {
                        let mut state = state::STATE.data.lock().unwrap();
                        state.time = time;

                        tauri_react::update_state(&mut webview, (*state).clone()).unwrap();
                    }

                    thread::sleep(Duration::from_secs(1));
                }
            });
        })
        .invoke_handler(|webview, arg| command_handler::<state::AppState>(webview, arg, &state::STATE, &commands::COMMANDS))
        .build()
        .run();
}
