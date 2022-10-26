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

use crate::tuple;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::fs::{canonicalize, DirEntry, ReadDir};
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

tuple!(CanonicalFsOp(source: PathBuf, target: PathBuf));
tuple!(FsOp(source: String, target: String));

impl FsOp {
    pub fn canonicalize(&self) -> CanonicalFsOp {
        let source = canonicalize_any(&PathBuf::from(&self.source));
        let target = canonicalize_any(&PathBuf::from(&self.target));

        (source, target).into()
    }
}

/// `std::fs::canonicalize()` does not work for paths that don't exist
fn canonicalize_any(path: &Path) -> PathBuf {
    let mut new_path = PathBuf::new();
    let mut have_root = false;

    for c in path.components() {
        match c {
            Component::RootDir => {
                if !have_root {
                    new_path.push(c.as_os_str());
                    have_root = true;
                } else {
                    panic!("Two roots in path {:?}", path);
                }
            }
            Component::CurDir => {}
            Component::ParentDir => {
                new_path.pop();
            }
            Component::Normal(_) | Component::Prefix(_) => new_path.push(c.as_os_str()),
        }
    }

    new_path
}

pub fn copy_file_with_relative_root(
    source_root: &Path,
    source_file: &Path,
    destination_root: &Path,
) {
    let canonical_source_file = canonicalize_any(source_file);
    let canonical_destination_file = change_root(source_root, source_file, destination_root);

    // make sure folder exists
    if let Some(parent) = canonical_destination_file.parent() {
        fs::create_dir_all(&parent)
            .unwrap_or_else(|err| panic!("Error creating directory {:?}\n{}", parent, err));
    }

    //println!(
    //    "{:?} to {:?}",
    //    canonical_source_file, canonical_destination_file
    //);
    let from_file_modified_time = fs::metadata(&canonical_source_file)
        .unwrap_or_else(|err| {
            panic!(
                "Error getting metadata from file {:?}\n{}",
                canonical_destination_file, err
            )
        })
        .modified()
        .unwrap_or_else(|err| {
            panic!(
                "Error getting modified time from file {:?}\n{}",
                canonical_destination_file, err
            )
        });
    let to_file_modified_time = fs::metadata(&canonical_destination_file)
        .map(|f| f.modified().unwrap_or(SystemTime::UNIX_EPOCH))
        .unwrap_or(SystemTime::UNIX_EPOCH);

    if from_file_modified_time < to_file_modified_time {
        return;
    }

    let mut from = fs::File::open(&canonical_source_file).unwrap_or_else(|err| {
        panic!(
            "Error opening file {:?}\n{}",
            canonical_destination_file, err
        )
    });
    let mut to = fs::File::create(&canonical_destination_file).unwrap_or_else(|err| {
        panic!(
            "Error creating file {:?}\n{}",
            canonical_destination_file, err
        )
    });

    std::io::copy(&mut from, &mut to).unwrap_or_else(|err| {
        panic!(
            "Error copying file {:?} to {:?}\n{}",
            canonical_source_file, canonical_destination_file, err
        )
    });
    // using std::fs::copy will copy permission bits as well, which may make copied files readonly,
    // which means we get 'Access denied' the next time we try to copy them
    //fs::copy(canonical_source_file, canonical_destination_file).unwrap();
}

pub fn change_root(
    source_root: &Path,
    source_item_path: &Path,
    destination_root: &Path,
) -> PathBuf {
    let canonical_source_root = canonicalize_any(source_root);
    let canonical_source_item = canonicalize_any(source_item_path);

    let relative_path = canonical_source_item.clone();
    let relative_path = relative_path
        .strip_prefix(&canonical_source_root)
        .unwrap_or_else(|err| {
            panic!(
                "Failed to strip prefix '{:?}' from path:\n{}",
                canonical_source_root, err
            )
        });

    let mut destination_path = PathBuf::from(destination_root);
    destination_path.push(relative_path);
    canonicalize_any(&destination_path)
}
