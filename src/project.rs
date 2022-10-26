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

use aeon::convert_panic::*;
use aeon::*;
use aeon_derive::{Deserialize, Serialize};
use std::path::PathBuf;
//use std::path::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectDir {
    pub root: String, // TODO: change to OsString asap
    pub profiles: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Projects {
    /// Profile name
    pub name: String,
    /// Source folder
    pub source: String,
    /// Destination to sync to
    pub destination: String,
    /// Ignored project directories
    pub ignored_projects: Vec<String>,
    /// Project directories and their sync profiles
    pub projects: Vec<ProjectDir>,
}

impl Projects {
    pub fn save(&self, file: PathBuf) -> std::io::Result<()> {
        let aeon = self.to_aeon();
        std::fs::write(&file, aeon)?;
        Ok(())
    }

    pub fn load(file: PathBuf) -> Self {
        let data = std::fs::read_to_string(&file)
            .unwrap_or_else(|_| panic!("Failed to read file '{:?}'", file));

        Self::from_aeon(data)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectFiles {
    pub file_names: Vec<String>,
}
impl ProjectFiles {
    pub fn save(&self, file: PathBuf) -> std::io::Result<()> {
        let aeon = self.to_aeon();
        std::fs::write(&file, aeon)?;
        Ok(())
    }

    pub fn load(file: PathBuf) -> Self {
        let data = std::fs::read_to_string(&file)
            .unwrap_or_else(|_| panic!("Failed to read file '{:?}'", file));

        Self::from_aeon(data)
    }
}

pub fn remove_project(project_name: &str) -> Result<(), std::io::Error> {
    let project_name = project_name.to_string();
    let project_name = if project_name.contains('*') {
        project_name
    } else {
        crate::util::add_extension_if_missing(project_name, ".aeon")
    };
    let mut projects = crate::util::load_project_list();
    let project_file = projects
        .file_names
        .iter()
        .find(|file| globber::glob_match(&project_name, file).unwrap());

    let project_file = match project_file {
        Some(f) => f,
        None => {
            println!("Could not find project {}", project_name);
            return Ok(());
        }
    };

    let remove_path = crate::util::get_project_file_path(project_file);
    if !crate::util::confirm(&format!("Remove project {:?}? Y/n", remove_path)) {
        return Ok(());
    }

    std::fs::remove_file(&remove_path)?;

    if let Some(index) = projects.file_names.iter().position(|f| f == project_file) {
        projects.file_names.swap_remove(index);
        crate::util::save_project_list(projects)?;
    }

    println!("Project removed: {:?}", remove_path);
    Ok(())
}
