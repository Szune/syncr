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

use crate::{util, Config, DirectoryTree, ProjectDir, Projects, ReadyConfig, ReadyProjectType};
use globber::glob_match_prebuilt;
use std::collections::HashMap;
use std::env;
use std::fs::DirEntry;

const UNDETERMINED: &str = "<undetermined>";

pub fn perform_init(init_name: Option<&str>) -> std::io::Result<()> {
    let init_name = init_name.map(|name| name.to_string());
    if util::get_cwd_string() == util::get_bin_dir_path() {
        println!(
            "Cannot sync the syncer (at least for now), sync project name would have been '{}'",
            init_name.unwrap_or_else(util::get_cwd_name)
        );
        return Ok(());
    }

    let mut project_files = util::load_project_list();
    let name = init_name.unwrap_or_else(util::get_cwd_name);
    let name_with_ext = util::add_extension_if_missing(name.clone(), ".aeon");
    let name = if !project_files.file_names.contains(&name_with_ext) {
        name
    } else {
        // TODO: check if the path is the same and if it is, skip init
        {
            let project = util::load_project(&name_with_ext);
            if std::path::PathBuf::from(project.source)
                == std::path::PathBuf::from(util::get_cwd_string())
            {
                println!("Project has already been initialized. Use the 'refresh' command to update the project file.");
                return Ok(());
            }
        }
        // TODO: check for all numbered names if the path is the same and if it is, skip init

        let mut num = 2;
        let tmp_name = loop {
            let mut tmp_name = name.clone();
            tmp_name.push('_');
            tmp_name.push_str(&num.to_string());
            let tmp_name_with_ext = util::add_extension_if_missing(tmp_name.clone(), ".aeon");
            if !project_files.file_names.contains(&tmp_name_with_ext) {
                break tmp_name;
            }
            num += 1;
        };
        if util::confirm(&format!(
            "{} already exists, create as {}? Y/n",
            name, tmp_name
        )) {
            tmp_name
        } else {
            return Ok(());
        }
    };

    let name_with_ext = util::add_extension_if_missing(name.clone(), ".aeon");

    println!("Initializing {}...", name);
    //println!("Syncr root: {}", util::get_bin_dir_path());
    // generate initial project file using either specified name or last part of path + ".aeon"
    // put the new file in the bin dir path when done
    let config: ReadyConfig = Config::load(util::get_config_file_path()).into();
    let mut project_dirs: Vec<DirEntry> = Vec::new();
    for f in DirectoryTree::new(env::current_dir().unwrap()) {
        let f = f?;
        if f.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            project_dirs.push(f);
        }
    }

    let mut projects = Vec::<ProjectDir>::new();

    let mut unable_to_determine = Vec::new();
    for e in project_dirs {
        if !determine_sync_project_dir(&e, &config.heuristics, &config, &mut projects)? {
            unable_to_determine.push(e.path());
        }
    }

    let projects = Projects {
        name,
        source: util::get_cwd_string(),
        destination: util::SORRY_MESSAGE.to_string(),
        ignored_projects: Vec::new(),
        projects,
    };

    let project_path = util::get_project_file_path(&name_with_ext);
    projects.save(project_path)?;

    for u in unable_to_determine {
        println!(
            "Unknown project type, using the '{}' profile for {:?}",
            UNDETERMINED, u
        );
    }

    project_files.file_names.push(name_with_ext);
    util::save_project_list(project_files)?;

    println!("Sync project initialized.");

    Ok(())

    // implement recursive iterator that goes breadth first instead of depth first
    // e.g. first reads through all the files in the current folder, then moves to deeper folder
    // unless that is what you have already implemented of course..
    // if that is the case, then implement recursive iterator that goes depth first, it's nice to
    // have option

    // step 1. use project heuristics to figure out folder project type
    // step 2. apply sync profile to folder
    // step 3. sync
    // step 4. back to 1 for next folder
}

fn determine_sync_project_dir(
    dir: &DirEntry,
    heuristics: &Vec<ReadyProjectType>,
    config: &ReadyConfig,
    projects: &mut Vec<ProjectDir>,
) -> std::io::Result<bool> {
    // println!("Syncing project directory: {:?}", dir.path());
    //let mut iter = DirectoryTree::recursive(dir.path().to_path_buf()); // dir_tree.rs
    let mut iter =
        DirectoryTree::recursive_skip_folders(dir.path(), config.heuristic_skip_folders.clone()); // dir_tree.rs

    let mut weights = HashMap::<&str, i32>::new();

    let mut last_dir_count = iter.remaining_dirs.len();
    let mut i = 0;
    while let Some(fsi) = iter.next() {
        // iterate file system items
        if i > 6 {
            // use --depth NUM to both toggle this and set the value, if --depth isn't used, it will go through all subfolders
            break;
        }

        let path = fsi?.path();
        // let path = match fsi {
        //     Ok(entry) => entry.path(),
        //     Err(e) => {
        //         println!("! IO error: {:?}", e);
        //         continue;
        //     },
        // };

        let file_name = match path.file_name() {
            Some(f) => f,
            None => {
                println!("! No name found for path {:?}", path);
                continue;
            }
        };

        let lossy = file_name.to_string_lossy();

        for proj_type in heuristics {
            let weight = if path.is_dir() {
                proj_type
                    .folders
                    .iter()
                    .filter(|h| glob_match_prebuilt(&h.value, &lossy))
                    .map(|h| h.weight)
                    .sum::<i32>()
            } else if path.is_file() {
                proj_type
                    .files
                    .iter()
                    .filter(|h| glob_match_prebuilt(&h.value, &lossy))
                    .map(|h| h.weight)
                    .sum::<i32>()
            } else {
                0
            };
            if weight > 0 {
                *weights.entry(proj_type.name.as_str()).or_insert(0) += weight;
            }
        }

        if iter.remaining_dirs.len() < last_dir_count {
            i += 1;
        }
        last_dir_count = iter.remaining_dirs.len();
    }

    let mut profiles = Vec::new();

    for (k, v) in weights.iter() {
        if v >= &30 {
            profiles.push(k.to_string());
        }
    }

    let determined = !profiles.is_empty();

    if determined {
        let root = format!("{}", dir.path().file_name().unwrap().to_string_lossy());
        println!("+ [{}]", root);
        for v in &profiles {
            println!("  * {}", v);
        }
        projects.push(ProjectDir { root, profiles });
    } else {
        let root = format!("{}", dir.path().file_name().unwrap().to_string_lossy());
        println!("+ [{}]", root);
        println!("  * {}", UNDETERMINED);
        projects.push(ProjectDir {
            root,
            profiles: vec![UNDETERMINED.to_owned()],
        });
    }
    Ok(determined)
}
