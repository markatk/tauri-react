/*
 * File: commands.rs
 * Author: MarkAtk
 * Date: 17.12.20
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
use once_cell::sync::Lazy;
use crate::state::AppState;

fn connect(mut state: AppState, _data: serde_json::Value) -> tauri::Result<AppState> {
    state.connected = true;

    Ok(state)
}

pub static COMMANDS: Lazy<HashMap<String, Box<dyn Fn(AppState, serde_json::Value) -> tauri::Result<AppState> + Send + Sync + 'static>>> = Lazy::new(|| {
    let mut c: HashMap<String, Box<dyn Fn(AppState, serde_json::Value) -> tauri::Result<AppState> + Send + Sync + 'static>> = HashMap::new();

    c.insert("connect".to_string(), Box::new(connect));

    c
});
