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

pub fn perform_refresh(refresh_name: Option<&str>) -> std::io::Result<()> {
    todo!();

    Ok(())
    //let config: Config = Config::load("config.aeon");
    //let config: ReadyConfig = config.into();

    //let heuristics = &config.heuristics;

    ////println!("[Config]\nSource: {:?}\nDestination: {:?}\nHeuristics skip folders: {:?}\n", config.source, config.destination, config.heuristic_skip_folders);

    ////println!("[Profile heuristics]\n{}\n",
    ////         config.heuristics.iter()
    ////             .map(|p|format!("{:?}",p))
    ////             .collect::<Vec<String>>()
    ////             .join("\n"));

    ////println!("[Sync profiles]\n{:?}\n{}\n", config.global_profile,
    ////         config.profiles.iter()
    ////             .map(|p|format!("{:?}",p))
    ////             .collect::<Vec<String>>()
    ////             .join("\n"));
    //println!("[Refresh started]");

    //let mut project_dirs: Vec<DirEntry> = Vec::new();
    //for f in DirectoryTree::new(config.source.clone().into()) {
    //    let f = f?;
    //    if f.file_type().map(|t| t.is_dir()).unwrap_or(false) {
    //        project_dirs.push(f);
    //    }
    //}

    //// let project_dirs =
    ////     x?.file_type().map(|t|t.is_dir()).unwrap_or(false)
    //// }.collect::<Vec<DirEntry>>();
    ////DirectoryTree::new(config.source.clone().into())
    ////.map(|f|f?)
    //// could use filter_map if we don't care about errors, doing it this way to not hide errors for now
    ////.filter(|x| x.file_type().map(|t|t.is_dir()).unwrap_or(false))
    ////.collect::<Vec<DirEntry>>();

    //let mut projects = Vec::<ProjectDir>::new();

    //let mut unable_to_determine = Vec::new();
    //for e in project_dirs {
    //    if !determine_sync_project_dir(&e, &heuristics, &config, &mut projects)? {
    //        unable_to_determine.push(e.path());
    //    }
    //}

    ////println!("Projects found:\n{:#?}", projects);

    //let projects = Projects { projects };
    //projects.save("projects.aeon")?;

    //for u in unable_to_determine {
    //    println!("Unable to determine project type, will not sync {:?}", u);
    //}

    //Ok(())

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
