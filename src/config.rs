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
use globber::*;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub heuristic_skip_folders: Vec<String>,
    pub global_profile: SyncProfile,
    pub profiles: Vec<SyncProfile>,
    pub heuristics: Vec<ProjectType>,
}

impl Config {
    pub fn load(file: PathBuf) -> Self {
        let data = std::fs::read_to_string(&file)
            .unwrap_or_else(|_| panic!("Failed to read file '{:?}'", file));

        Config::from_aeon(data)
        //println!("{:#?}", config);
    }
}

#[derive(Debug)]
pub struct ReadyConfig {
    pub heuristic_skip_folders: GlobList,
    pub global_profile: ReadySyncProfile,
    pub profiles: Vec<ReadySyncProfile>,
    pub heuristics: Vec<ReadyProjectType>,
}

impl From<Config> for ReadyConfig {
    fn from(conf: Config) -> ReadyConfig {
        ReadyConfig {
            heuristic_skip_folders: GlobList::build(&conf.heuristic_skip_folders).unwrap(),
            global_profile: conf.global_profile.into(),
            profiles: conf.profiles.into_iter().map(|it| it.into()).collect(),
            heuristics: conf.heuristics.into_iter().map(|it| it.into()).collect(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SyncProfile {
    /// Profile name
    pub name: String,
    /// Folders to skip
    pub skip_folders: Vec<String>,
    /// File patterns to skip
    pub skip_files: Vec<String>,
}

#[derive(Debug)]
pub struct ReadySyncProfile {
    /// Profile name
    pub name: String,
    /// Folders to skip
    pub skip_folders: GlobList,
    /// File patterns to skip
    pub skip_files: GlobList,
}

impl From<SyncProfile> for ReadySyncProfile {
    fn from(sync_profile: SyncProfile) -> ReadySyncProfile {
        let skip_folders = GlobList::build(&sync_profile.skip_folders).unwrap();
        let skip_files = GlobList::build(&sync_profile.skip_files).unwrap();
        ReadySyncProfile {
            name: sync_profile.name,
            skip_folders,
            skip_files,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Heuristic {
    pub value: String,
    pub weight: i32,
}

#[derive(Debug)]
pub struct ReadyHeuristic {
    pub value: GlobPattern,
    pub weight: i32,
}

impl From<Heuristic> for ReadyHeuristic {
    fn from(heuristic: Heuristic) -> ReadyHeuristic {
        let pattern = build_glob_pattern(heuristic.value.as_str()).unwrap();
        ReadyHeuristic {
            value: pattern,
            weight: heuristic.weight,
        }
    }
}

/// Heuristics to determine project type
#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectType {
    pub name: String,
    /// Folders likely present in project type
    pub folders: Vec<Heuristic>,
    /// Files likely present in project type
    pub files: Vec<Heuristic>,
}

#[derive(Debug)]
pub struct ReadyProjectType {
    pub name: String,
    /// Folders likely present in project type
    pub folders: Vec<ReadyHeuristic>,
    /// Files likely present in project type
    pub files: Vec<ReadyHeuristic>,
}

impl From<ProjectType> for ReadyProjectType {
    fn from(pt: ProjectType) -> ReadyProjectType {
        let folders = pt.folders.into_iter().map(|it| it.into()).collect();
        let files = pt.files.into_iter().map(|it| it.into()).collect();
        ReadyProjectType {
            name: pt.name,
            folders,
            files,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Heuristics {
    heuristics: Vec<ProjectType>,
}

#[derive(Debug)]
pub struct ReadyHeuristics {
    heuristics: Vec<ReadyProjectType>,
}

impl From<Heuristics> for ReadyHeuristics {
    fn from(heuristics: Heuristics) -> ReadyHeuristics {
        ReadyHeuristics {
            heuristics: heuristics
                .heuristics
                .into_iter()
                .map(|it| it.into())
                .collect(),
        }
    }
}
