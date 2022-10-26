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

use globber::*;
use std::collections::*;
use std::fs::{DirEntry, ReadDir};
use std::path::*;

/// Does not take symlinks into account at this time.
pub struct DirectoryTree {
    pub root: PathBuf,
    current_dir_iter: Option<ReadDir>,
    pub remaining_dirs: VecDeque<PathBuf>,
    recursive_skip_folders: GlobList,
    recursive_skip_files: GlobList,
    recursive: bool,
}

impl DirectoryTree {
    pub fn new(root: PathBuf) -> DirectoryTree {
        let mut remaining = VecDeque::new();
        remaining.push_back(root.clone());
        DirectoryTree {
            root,
            current_dir_iter: None,
            remaining_dirs: remaining,
            recursive_skip_folders: GlobList::new(),
            recursive_skip_files: GlobList::new(),
            recursive: false,
        }
    }

    pub fn recursive(root: PathBuf) -> DirectoryTree {
        let mut remaining = VecDeque::new();
        remaining.push_back(root.clone());
        DirectoryTree {
            root,
            current_dir_iter: None,
            remaining_dirs: remaining,
            recursive_skip_folders: GlobList::new(),
            recursive_skip_files: GlobList::new(),
            recursive: true,
        }
    }

    pub fn recursive_skip_folders(root: PathBuf, folders: GlobList) -> DirectoryTree {
        let mut remaining = VecDeque::new();
        remaining.push_back(root.clone());
        DirectoryTree {
            root,
            current_dir_iter: None,
            remaining_dirs: remaining,
            recursive_skip_folders: folders,
            recursive_skip_files: GlobList::new(),
            recursive: true,
        }
    }

    pub fn recursive_skip_folders_and_files(
        root: PathBuf,
        folders: GlobList,
        files: GlobList,
    ) -> DirectoryTree {
        let mut remaining = VecDeque::new();
        remaining.push_back(root.clone());
        DirectoryTree {
            root,
            current_dir_iter: None,
            remaining_dirs: remaining,
            recursive_skip_folders: folders,
            recursive_skip_files: files,
            recursive: true,
        }
    }

    pub fn skip_folders_and_files(
        root: PathBuf,
        folders: GlobList,
        files: GlobList,
    ) -> DirectoryTree {
        let mut remaining = VecDeque::new();
        remaining.push_back(root.clone());
        DirectoryTree {
            root,
            current_dir_iter: None,
            remaining_dirs: remaining,
            recursive_skip_folders: folders,
            recursive_skip_files: files,
            recursive: false,
        }
    }
}

impl Iterator for DirectoryTree {
    type Item = Result<DirEntry, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // note to self: deque as a queue -> push_back() == enqueue() and pop_front() == dequeue()

        macro_rules! pop_remaining (
            () => (
                if let Some(dir) = self.remaining_dirs.pop_front() {
                    self.current_dir_iter = Some(dir.read_dir().ok()?);
                } else {
                    return None;
                }
                )
            );

        // TODO: rewrite to be readable
        let next_item = loop {
            if let Some(ref mut curr_dir) = self.current_dir_iter {
                if let Some(current) = curr_dir.next() {
                    if let Ok(ref it) = current {
                        if it.path().is_file() {
                            let skip_file = it
                                .path()
                                .file_name()
                                .map(|f| self.recursive_skip_files.any_match(&f.to_string_lossy()))
                                .unwrap_or(true); // if the file has no discernible name, skip it
                            if skip_file {
                                continue;
                            } else {
                                break current;
                            }
                        } else if self.recursive && it.path().is_dir() {
                            let skip_folder = it
                                .path()
                                .file_name()
                                .map(|d| {
                                    self.recursive_skip_folders.any_match(&d.to_string_lossy())
                                })
                                .unwrap_or(true); // if the folder has no discernible name, skip it
                            if skip_folder {
                                continue;
                            } else {
                                // didn't match any skip folders
                                self.remaining_dirs.push_back(it.path());
                            }
                        }
                    }
                    break current;
                }
            }
            // note: pop_remaining!(); returns None if there's nothing left to traverse,
            // that's why we always return Some(next_item) at the end
            pop_remaining!();
        };

        Some(next_item)
    }
}
