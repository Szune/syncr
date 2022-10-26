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

use config::*;
use dir_tree::DirectoryTree;
use project::*;
use std::env;

pub(crate) mod args;
pub(crate) mod config;
pub(crate) mod dir_tree;
pub(crate) mod fs;
pub(crate) mod init;
pub(crate) mod macros;
pub(crate) mod project;
pub(crate) mod refresh;
pub(crate) mod sync;
pub(crate) mod util;

fn main() -> Result<(), std::io::Error> {
    // use heuristics to generate a sync.aeon file to use as a starting point
    // execute syncr --gen to generate
    // execute syncr to sync using sync.aeon predefined profiles and paths
    // report directories and files in the main sync dir that haven't been added to the profile yet

    let args: Vec<String> = env::args().skip(1).collect();
    let args: &[&str] = &args.iter().map(|a| a.as_str()).collect::<Vec<&str>>()[..];
    args::handle_args(args)
}
