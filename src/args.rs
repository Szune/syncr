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

use crate::config::Config;
use crate::util;

macro_rules! arg_list {
        (input: $input:expr; args: $($arg:ident $(as $arg_name:literal)?),* $(,)?) => {
            match $input {
                $(
                [stringify!($arg), rest @ ..]
                $(
                | [$arg_name, rest @ ..]
                )* => $arg::handle_args(rest)?,
                )*

                $(
                ["--help", stringify!($arg), ..] | ["help", stringify!($arg), ..]
                $(
                | ["--help", $arg_name, ..] | ["help", $arg_name, ..]
                )* => $arg::print_full_help(),
                )*
                ["--help", ..] | ["help", ..] => {
                    println!("Syncr {}\n", env!("CARGO_PKG_VERSION"));
                    println!("args:");
                    $(
                    $arg::print_cmd_help();
                    )*
                    println!("  help, --help\n  \tprint help text");
                }
                [] => {
                    println!("Syncr requires at least one arg\n");
                    println!("args:");
                    $(
                    $arg::print_cmd_help();
                    )*
                    println!("  help, --help\n  \tprint help text");
                }
                remaining => {
                    println!("Unknown args: '{}'\n", remaining.join("', '"));
                    println!("args:");
                    $(
                    $arg::print_cmd_help();
                    )*
                    println!("  help, --help\n  \tprint help text");
                }
            }
            Ok(())
        };
    }

pub fn handle_args(args: &[&str]) -> Result<(), std::io::Error> {
    arg_list! {
        input: args;
        args: config, init, list, refresh, remove, show, sync, version
    }
}

macro_rules! arg_cmd {
    (cmd $cmd_id:ident $(as $cmd_name:literal)?:
     {
     main will $main_help:literal $main_block:block
   $(arg $arg_id:ident $(as $arg_name:literal)? will $arg_help:literal $arg_block:block )*
   $(subcmd $sub_cmd:ident $(as $sub_cmd_name:literal)? can $sub_cmd_help:literal $sub_cmd_block:tt)*
     }) => {
        arg_cmd!{
            cmd $cmd_id $(as $cmd_name)*: main will $main_help $main_block
            $(arg $arg_id $(as $arg_name)* will $arg_help $arg_block)*
            $(subcmd $sub_cmd $(as $sub_cmd_name)* can $sub_cmd_help $sub_cmd_block)*
        }
    };

    (cmd $cmd_id:ident $(as $cmd_name:literal)?:
     main will $main_help:literal $main_block:block
   $(arg $arg_id:ident $(as $arg_name:literal)? will $arg_help:literal $arg_block:block )*
   $(subcmd $sub_cmd:ident $(as $sub_cmd_name:literal)? can $sub_cmd_help:literal $sub_cmd_block:tt)*
    ) => {
        #[allow(non_camel_case_types)]
        mod $cmd_id {

            $(
            arg_cmd!{cmd $sub_cmd $(as $sub_cmd_name)*: $sub_cmd_block}
            )*

            pub fn print_full_help() {
                #[allow(unused_assignments,unused_variables)]
                let have_sub_cmds = false;
                $(
                _ = $sub_cmd_help;
                #[allow(unused_assignments,unused_variables)]
                let have_sub_cmds = true;
                )*
                if have_sub_cmds {
                    println!("commands:");
                    $(
                    self::$sub_cmd::print_cmd_help();
                    //println!("  {}\n  \t{}", stringify!($sub_cmd), $sub_cmd_help);
                    )*
                    println!();
                }
                #[allow(unused_assignments)]
                let cmd_name = stringify!($cmd_id).trim_start_matches("r#");
                $(
                let cmd_name = $cmd_name;
                )*
                println!("args:");
                println!("  {}\n  \t{}", cmd_name, $main_help);
                $(
                #[allow(unused_assignments)]
                let arg_name = stringify!($arg_id);
                $(
                let arg_name = $arg_name;
                )*
                println!("  {} {}\n  \t{}", cmd_name, arg_name, $arg_help);
                )*
                println!("  {} help, --help\n  \tprint help text", cmd_name);
            }

            pub fn print_cmd_help() {
                #[allow(unused_assignments)]
                let cmd_name = stringify!($cmd_id).trim_start_matches("r#");
                $(
                let cmd_name = $cmd_name;
                )*
                println!("  {}\n  \t{}", cmd_name, $main_help);
            }

            pub fn handle_args(args: &[&str]) -> Result<(), std::io::Error> {
                match args {
                    $(
                    [stringify!($sub_cmd), rest @ ..]
                    $(
                    | [$sub_cmd_name, rest @ ..]
                    )* => self::$sub_cmd::handle_args(rest)?,
                    )*
                    $(
                    [stringify!($arg_id)]
                    $(
                    | [$arg_name]
                    )* => $arg_block,
                    )*
                    ["--help", ..] | ["help", ..] => {
                        println!("Syncr {}\n", env!("CARGO_PKG_VERSION"));
                        self::print_full_help();
                    }
                    [] => $main_block,
                    remaining => {
                        #[allow(unused_assignments)]
                        let cmd_name = stringify!($cmd_id).trim_start_matches("r#");
                        $(
                        let cmd_name = $cmd_name;
                        )*
                        println!("Unknown args to {}: '{}'\n", cmd_name, remaining.join("', '"));
                        self::print_full_help();
                    }
                }
                Ok(())
            }
        }
    };

    (cmd $cmd_id:ident $(as $cmd_name:literal)?:
     {
     main with
   $(required value $required_id:ident $(as $required_name:literal)?)?
   $(optional value $optional_id:ident $(as $optional_name:literal)?)?
     will $main_help:literal $main_call:tt
   $(arg $arg_id:ident $(as $arg_name:literal)? will $arg_help:literal $arg_block:block )*
     }) => {
        arg_cmd!{
            cmd $cmd_id $(as $cmd_name)*:
            main with
           $(required value $required_id $(as $required_name)*)*
           $(optional value $optional_id $(as $optional_name)*)*
            will $main_help $main_call
            $(arg $arg_id $(as $arg_name)* will $arg_help $arg_block)*
        }
    };

    (cmd $cmd_id:ident $(as $cmd_name:literal)?:
     main with
   $(required value $required_id:ident $(as $required_name:literal)?)?
   $(optional value $optional_id:ident $(as $optional_name:literal)?)?
     will $main_help:literal $main_call:tt
   $(arg $arg_id:ident $(as $arg_name:literal)? will $arg_help:literal $arg_block:block )*
    ) => {
        #[allow(non_camel_case_types)]
        mod $cmd_id {
            pub fn print_full_help() {
                #[allow(unused_assignments)]
                let cmd_name = stringify!($cmd_id).trim_start_matches("r#");
                $(
                let cmd_name = $cmd_name;
                )*
                println!("args:");
                $(
                #[allow(unused_assignments)]
                let required_name = stringify!($required_id).trim_start_matches("r#").to_uppercase();
                $(
                let required_name = $required_name.to_uppercase();
                )*
                println!("  {} {}\n  \t{}", cmd_name, required_name, $main_help);
                )*
                $(
                #[allow(unused_assignments)]
                let optional_name = stringify!($optional_id).trim_start_matches("r#").to_uppercase();
                $(
                let optional_name = $optional_name.to_uppercase()
                )*
                println!("  {} [{}]\n  \t{}", cmd_name, optional_name, $main_help);
                )*
                $(
                #[allow(unused_assignments)]
                let arg_name = stringify!($arg_id);
                $(
                let arg_name = $arg_name;
                )*
                println!("  {} --{}\n  \t{}", cmd_name, arg_name, $arg_help);
                )*
                println!("  {} --help\n  \tprint help text", cmd_name);
            }

            pub fn print_cmd_help() {
                #[allow(unused_assignments)]
                let cmd_name = stringify!($cmd_id).trim_start_matches("r#");
                $(
                let cmd_name = $cmd_name;
                )*
                println!("  {}\n  \t{}",cmd_name, $main_help);
            }

            pub fn handle_args(args: &[&str]) -> Result<(), std::io::Error> {
                match args {
                    $(
                    [concat!("--", stringify!($arg_id))]
                    $(
                    | [concat!("--", $arg_name)]
                    )* => $arg_block,
                    )*
                    ["--help", ..] => {
                        println!("Syncr {}\n", env!("CARGO_PKG_VERSION"));
                        self::print_full_help();
                    }
                    $(
                    [] => {
                        let $optional_id = None;
                        $main_call;
                    },
                    [value] => {
                        let $optional_id = Some(*value);
                        $main_call;
                    },
                    )*
                    $(
                    [value] => {
                        let $required_id = *value;
                        $main_call;
                    },
                    )*
                    remaining => {
                        #[allow(unused_assignments)]
                        let cmd_name = stringify!($cmd_id).trim_start_matches("r#");
                        $(
                        let cmd_name = $cmd_name;
                        )*
                        println!("Unknown args to {}: '{}'\n", cmd_name, remaining.join("', '"));
                        self::print_full_help();
                    }
                }
                Ok(())
            }
        }
    };
}

arg_cmd! {
    cmd config:
    main will "print config.aeon" {
        crate::args::print_config();
    }
    subcmd heuristics can "print heuristics" {
        main will "print full heuristics" {
            crate::args::print_config_section("heuristics");
        }
        arg names will "print only heuristic names" {
            let (_, heuristics) = crate::args::get_heuristics();
            heuristics.iter().for_each(|h|println!("{}", h.name));
        }
    }
    subcmd profiles can "print profiles" {
        main will "print full profiles" {
            crate::args::print_config_section("profiles");
        }
        arg global will "print global profile" {
            let (global, _) = crate::args::get_sync_profiles();
            println!("{}",
                format_args!(
                    "{{ name: {:?},\n  skip_folders: {:?},\n  skip_files: {:?}\n}}",
                    global.name,
                    global.skip_folders,
                    global.skip_files
                ),
            );
        }
        arg names will "print only profile names" {
            let (_, profiles) = crate::args::get_sync_profiles();
            profiles.iter().for_each(|p|println!("{}", p.name));
        }
    }
}

arg_cmd! {
    cmd init:
    main with optional value r#true will "initialize sync project in current dir" {
        crate::init::perform_init(r#true)?;
    }
}

arg_cmd! {
    cmd list:
    main will "print project_files.aeon" {
        crate::args::print_project_list();
    }
    arg names will "print only file names" {
        crate::args::print_project_list_names();
    }
}

arg_cmd! {
    cmd refresh:
    main with optional value name will "refreshes all projects or specified project (update project folder list)" {
        crate::refresh::perform_refresh(name)?;
    }
    arg all will "refresh all sync projects" {
        println!("not doing anything, TODO: do refresh all");
    }
}

arg_cmd! {
    cmd remove:
    main with required value name will "remove a specific sync project (not synced files)" {
        crate::project::remove_project(name)?;
    }
}

arg_cmd! {
    cmd show:
    main will "print all projects in full" {
        crate::args::print_projects(None)?;
    }
    subcmd full can "print project in full" {
        main with required value name will "print project in full" {
            crate::args::print_projects(Some(name))?;
        }
    }
    subcmd paths can "print project paths" {
        main with optional value name will "print all or specific project paths (source and destination)" {
            crate::args::print_project_paths(name)?;
        }
    }
}

arg_cmd! {
    cmd sync:
    main with optional value name will "sync the current directory or the specified sync project" {
        crate::sync::perform_sync(name)?;
    }
    arg all will "sync all sync projects" {
        println!("not doing anything, TODO: do sync all");
    }
}

arg_cmd! {
    cmd version:
    main will "print version number" {
        println!("{}", env!("CARGO_PKG_VERSION"));
    }
}

fn print_project_list_names() {
    let projects = util::load_project_list();
    projects.file_names.iter().for_each(|f| println!("{}", f));
}

fn print_config() {
    let config: Config = Config::load(util::get_config_file_path());
    //let config: ReadyConfig = config.into();
    let projects = util::load_project_list();

    //let heuristics = &config.heuristics;
    println!("[Config]\nProjects: [{}]", projects.file_names.join(", "));

    println!(
        "Heuristics skip folders: {:?}\n",
        config.heuristic_skip_folders
    );

    println!(
        "[Profile heuristics]\n{}\n",
        config
            .heuristics
            .iter()
            .map(|p| format!(
                "{{ name: {:?}, folders: [{}], files: [{}] }}",
                p.name,
                p.folders
                    .iter()
                    .map(|f| format!("{{ value: {:?}, weight: {} }}", f.value, f.weight))
                    .collect::<Vec<String>>()
                    .join(", "),
                p.files
                    .iter()
                    .map(|f| format!("{{ value: {:?}, weight: {} }}", f.value, f.weight))
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
            .collect::<Vec<String>>()
            .join("\n")
    );

    let profile_text = config
        .profiles
        .iter()
        .map(|p| {
            format!(
                "{{ name: {:?}, skip_folders: {:?}, skip_files: {:?} }}",
                p.name, p.skip_folders, p.skip_files
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!(
        "[Sync profiles]\n{}\n{}\n",
        format_args!(
            "{{ name: {:?}, skip_folders: {:?}, skip_files: {:?} }}",
            config.global_profile.name,
            config.global_profile.skip_folders,
            config.global_profile.skip_files
        ),
        profile_text
    );
}

fn print_config_section(section: &str) {
    let config: Config = Config::load(util::get_config_file_path());

    match section {
        "heuristics" => {
            println!(
                "[Global]\nHeuristics skip folders: {:?}\n",
                config.heuristic_skip_folders
            );

            println!(
                "[Profile heuristics]\n{}",
                config
                    .heuristics
                    .iter()
                    .map(|p| format!(
                        "{{ name: {:?},\n  folders: [{}],\n  files: [{}]\n}}",
                        p.name,
                        p.folders
                            .iter()
                            .map(|f| format!("{{ value: {:?}, weight: {} }}", f.value, f.weight))
                            .collect::<Vec<String>>()
                            .join(", "),
                        p.files
                            .iter()
                            .map(|f| format!("{{ value: {:?}, weight: {} }}", f.value, f.weight))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        }
        "profiles" => {
            println!(
                "[Global]\n{}\n",
                format_args!(
                    "{{ name: {:?}, skip_folders: {:?}, skip_files: {:?} }}",
                    config.global_profile.name,
                    config.global_profile.skip_folders,
                    config.global_profile.skip_files
                ),
            );

            let profile_text = config
                .profiles
                .iter()
                .map(|p| {
                    format!(
                        "{{ name: {:?},\n  skip_folders: {:?},\n  skip_files: {:?}\n}}",
                        p.name, p.skip_folders, p.skip_files
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            println!("[Sync profiles]\n{}", profile_text);
        }
        _ => unreachable!(),
    }
}

/// (Heuristic skip folders, heuristics)
fn get_heuristics() -> (Vec<String>, Vec<crate::config::ProjectType>) {
    let config: Config = Config::load(util::get_config_file_path());
    (config.heuristic_skip_folders, config.heuristics)
}

/// (Global profile, Sync profiles)
fn get_sync_profiles() -> (crate::config::SyncProfile, Vec<crate::config::SyncProfile>) {
    let config: Config = Config::load(util::get_config_file_path());
    (config.global_profile, config.profiles)
}

fn print_project_list() {
    let projects = util::load_project_list();
    println!("[Projects]");
    if !projects.file_names.is_empty() {
        println!(
            "project_files: [\n  \"{}\",\n]",
            projects.file_names.join("\",\n  \"")
        );
    } else {
        println!("project_files: []");
    }
}

fn print_projects(specific: Option<&str>) -> std::io::Result<()> {
    let specific = specific.map(|name| name.to_string());
    let projects = util::load_project_list();
    if specific.is_none() {
        println!("[Projects]\n");
    }

    specific
        .map(|x| vec![x])
        .unwrap_or_else(|| projects.file_names)
        .into_iter()
        .for_each(|p| {
            let p = util::add_extension_if_missing(p, ".aeon");

            let projects = util::load_project(&p);

            println!(
                "[{}]\nSource: {:?}\nDestination: {:?}\nIgnored projects:\n{}\n\nProjects:\n{}",
                projects.name,
                projects.source,
                projects.destination,
                projects
                    .ignored_projects
                    .iter()
                    .map(|p| format!("{{ \"{}\" }}", p))
                    .collect::<Vec<String>>()
                    .join("\n"),
                projects
                    .projects
                    .iter()
                    .map(|p| format!("{{ root: {:?}, profiles: {:?} }}", p.root, p.profiles))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        });
    Ok(())
}

fn print_project_paths(specific: Option<&str>) -> std::io::Result<()> {
    let specific = specific.map(|name| name.to_string());
    let projects = util::load_project_list();

    specific
        .map(|x| vec![x])
        .unwrap_or_else(|| projects.file_names)
        .into_iter()
        .for_each(|p| {
            let p = util::add_extension_if_missing(p, ".aeon");

            let projects = util::load_project(&p);

            println!(
                "[{}]\nSource: {:?}\nDestination: {:?}\n",
                projects.name, projects.source, projects.destination,
            );
        });
    Ok(())
}
