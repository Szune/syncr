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

use crate::fs;
use crate::{util, Config, DirectoryTree, ReadyConfig};
use globber::GlobList;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

pub fn copy(source_root: &Path, source: &DirEntry, destination_root: &PathBuf) {
    // TODO: copy with permission bits
    // TODO: remove permission bits of destination file, then do fs::copy
    let path = source.path();
    if path.is_dir() {
        let new_dir = fs::change_root(source_root, &path, destination_root);
        std::fs::create_dir_all(&new_dir)
            .unwrap_or_else(|err| panic!("Error creating directory {:?}\n{}", new_dir, err));
    } else if path.is_file() {
        fs::copy_file_with_relative_root(source_root, &path, destination_root);
    }
}

pub fn perform_sync(sync_name: Option<&str>) -> std::io::Result<()> {
    let sync_name = sync_name.map(|name| name.to_string());
    let name = sync_name.unwrap_or_else(util::get_cwd_name);
    let name_with_ext = util::add_extension_if_missing(name, ".aeon");
    let project_dir = util::load_project(&name_with_ext);
    if project_dir.destination.trim() == util::SORRY_MESSAGE {
        println!("Destination must be set before sync");
        return Ok(());
    }

    let config: ReadyConfig = Config::load(util::get_config_file_path()).into();

    let source = project_dir.source;
    let destination = project_dir.destination;

    let skip_folders = config.global_profile.skip_folders.clone();
    let skip_files = config.global_profile.skip_files.clone();

    let source_path = PathBuf::from(&source);
    let destination_path = PathBuf::from(&destination);

    DirectoryTree::skip_folders_and_files(source_path.clone(), skip_folders, skip_files)
        .filter_map(|f| f.ok())
        .filter_map(Some)
        .filter(|f| f.metadata().map(|m| m.is_file()).unwrap_or(false))
        .for_each(|f| copy(&source_path, &f, &destination_path));
    //.for_each(|f| println!("(project root) {:?}", f.path()));

    project_dir.projects.into_iter().for_each(|pd| {
        let (mut skip_folders, mut skip_files): (Vec<GlobList>, Vec<GlobList>) =
            pd.profiles.iter().fold(
                (Vec::new(), Vec::new()),
                |(mut skip_folders, mut skip_files), profile_name| {
                    let profile = config
                        .profiles
                        .iter()
                        .find(|profile| &profile.name == profile_name)
                        .unwrap_or_else(|| {
                            panic!("Profile '{}' does not exist in config.aeon", profile_name)
                        });
                    skip_folders.push(profile.skip_folders.clone());
                    skip_files.push(profile.skip_files.clone());
                    (skip_folders, skip_files)
                },
            );
        skip_folders.push(config.global_profile.skip_folders.clone());
        skip_files.push(config.global_profile.skip_files.clone());

        let skip_folders: GlobList = GlobList::combine(skip_folders);
        let skip_files: GlobList = GlobList::combine(skip_files);

        let mut path = PathBuf::from(&source);
        path.push(pd.root);
        DirectoryTree::recursive_skip_folders_and_files(path, skip_folders, skip_files)
            .filter_map(|f| f.ok())
            .filter_map(Some)
            .for_each(|f| copy(&source_path, &f, &destination_path));
        //.for_each(|f| println!("* {:?}", f.path()));
    });

    println!("Sync complete.");
    Ok(())
}
