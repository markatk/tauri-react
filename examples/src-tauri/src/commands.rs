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
use std::fs::{self, File};
use std::io::BufReader;
use std::io::prelude::*;
use once_cell::sync::Lazy;
use serde::Deserialize;
use tauri_react::INIT_FUNC;
use crate::state::AppState;

fn initialize(mut state: AppState, _: serde_json::Value) -> tauri::Result<AppState> {
    state.path = "todos.txt".to_string();
    state.todos.clear();

    // read from file
    let file = File::open(&state.path)?;
    for line in BufReader::new(file).lines() {
        if let Ok(todo) = line {
            state.todos.push(todo);
        }
    }

    Ok(state)
}

fn write_todos_to_file(todos: &Vec<String>, path: &str) -> Result<(), std::io::Error> {
    let data = todos.join("\n");
    fs::write(path, data.as_str())?;

    Ok(())
}

#[derive(Deserialize)]
struct AddTodoData {
    pub todo: String
}

fn add_todo(mut state: AppState, value: serde_json::Value) -> tauri::Result<AppState> {
    let data: AddTodoData = serde_json::from_value(value)?;
    state.todos.push(data.todo);

    write_todos_to_file(&state.todos, &state.path)?;

    Ok(state)
}

#[derive(Deserialize)]
struct DeleteTodoData {
    pub index: usize
}

fn delete_todo(mut state: AppState, value: serde_json::Value) -> tauri::Result<AppState> {
    let data: DeleteTodoData = serde_json::from_value(value)?;
    state.todos.remove(data.index);

    write_todos_to_file(&state.todos, &state.path)?;

    Ok(state)
}

pub static COMMANDS: Lazy<HashMap<String, Box<dyn Fn(AppState, serde_json::Value) -> tauri::Result<AppState> + Send + Sync + 'static>>> = Lazy::new(|| {
    let mut c: HashMap<String, Box<dyn Fn(AppState, serde_json::Value) -> tauri::Result<AppState> + Send + Sync + 'static>> = HashMap::new();

    c.insert(INIT_FUNC.to_string(), Box::new(initialize));
    c.insert("add-todo".to_string(), Box::new(add_todo));
    c.insert("delete-todo".to_string(), Box::new(delete_todo));

    c
});
