/*
 * File: state.rs
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

use std::sync::{Arc, Mutex, MutexGuard};
use once_cell::sync::Lazy;
use tauri_react::{ApplicationState, StoreState};
use serde::Serialize;

#[derive(Serialize, Default, Clone)]
pub struct AppState {
    pub todos: Vec<String>
}

impl ApplicationState for AppState {}

impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(todos={})", self.todos.join(";"))
    }
}

#[derive(Default)]
pub struct State {
    pub data: Mutex<AppState>
}

impl StoreState<AppState> for State {
    fn get_data(&self) -> tauri::Result<MutexGuard<AppState>> {
        // TODO: Proper error handling
        Ok(self.data.lock().unwrap())
    }
}

pub static STATE: Lazy<Arc<dyn StoreState<AppState>>> = Lazy::new(|| {
    Arc::new(State::default())
});
