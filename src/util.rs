//
// Syncr is a syncer
// Copyright (C) 2022  Carl Erik Patrik Iwarson
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

use crate::{ProjectFiles, Projects};
use std::io::Write;

pub const SORRY_MESSAGE: &str = "<CURRENTLY HAS TO BE FILLED IN MANUALLY SORRY>";

/// Gets the name of the current working directory
pub fn get_cwd_name() -> String {
    std::env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

/// Gets the full path to the current working directory as a String
pub fn get_cwd_string() -> String {
    std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

/// Gets the full path to the directory of the current binary
pub fn get_bin_dir_path() -> String {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

/// Gets the full path to the current binary
pub fn get_bin_exe_path() -> String {
    std::env::current_exe()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

pub fn get_config_file_path() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .with_file_name("config.aeon")
}

pub fn get_project_list_path() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .with_file_name("project_files.aeon")
}

pub fn get_project_file_path(project_name: &str) -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .with_file_name(project_name)
}

pub fn load_project(project_name: &str) -> Projects {
    let path = get_project_file_path(project_name);
    Projects::load(path)
}

pub fn load_project_list() -> ProjectFiles {
    let path = get_project_list_path();
    if path.exists() {
        ProjectFiles::load(path)
    } else {
        ProjectFiles {
            file_names: Vec::new(),
        }
    }
}

pub fn save_project_list(project_list: ProjectFiles) -> std::io::Result<()> {
    project_list.save(get_project_list_path())
}

pub fn add_extension_if_missing(s: String, extension: &str) -> String {
    if !s.ends_with(extension) {
        let mut s = s;
        s.push_str(extension);
        s
    } else {
        s
    }
}

pub fn confirm(prompt: &str) -> bool {
    println!("{}", prompt);
    print!(">");
    std::io::stdout().flush().unwrap();

    let mut result = String::new();
    std::io::stdin().read_line(&mut result).unwrap();

    return result.trim_end().to_ascii_uppercase() == "Y";
}
