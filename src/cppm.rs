#![allow(dead_code)]
mod builder;

use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
    process::{self, Command},
};

use colored::Colorize;
use fsio::file;
use serde::{Deserialize, Serialize};

use crate::build::run;

// Imports for `cppm --status`
use serde_json::Value;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    location: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalConfig {
    pub project: HashMap<String, String>,
}
pub fn write(project_name: &str, location: &str) {
    file::ensure_exists(&misc::configfile()).ok();

    let config: Config = Config {
        name: project_name.to_string(),
        location: location.replace('/', "\\"),
    };
    file::append_file(
        &misc::configfile(),
        format!("\n[[config]]\n{}", toml::to_string_pretty(&config).unwrap()).as_bytes(),
    )
    .expect("config not written to.");
}

pub mod misc {
    use crate::cppm::Config;
    use colored::Colorize;
    use std::collections::HashMap;
    use std::path::Path;

    pub const CBOILER: &str = r#"
#include <stdio.h>

int main(void) {
    printf("Hello, cppm!\n");
    return 0;
}
"#;

    pub fn header_boiler_c(header_name: &str) -> String {
        format!(
            r#"#ifndef {0}_H
#define {0}_H

#include <stdio.h>


#endif"#,
            header_name.to_uppercase().replace("-", "_")
        )
    }

    pub const CPPBOILER: &str = r#"#include <iostream>

int main(){

    std::cout << "Hello World" << std::endl;
    return 0;
}
"#;
    /// C++ header file boilerplate.
    pub fn header_boiler(header_name: &str) -> String {
        format!(
            r#"#pragma once

#ifndef {0}_HPP
#define {0}_HPP

#include <iostream>


#endif"#,
            header_name.to_uppercase().replace("-", "_")
        )
    }

    pub fn configfile() -> String {
        let configdir = dirs::home_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
            .replace('"', "")
            .replace('\\', "/");
        format!("{}/.cppm/config.toml", configdir)
    }

    pub fn dir_name() -> String {
        std::path::Path::new(&std::env::current_dir().unwrap())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn list_projects() {
        if !Path::new(&configfile()).exists() {
            println!("{}", "You have not created any projects yet!".red());
            std::process::exit(0);
        }
        let config: HashMap<String, Vec<Config>> =
            toml::from_str(&std::fs::read_to_string(configfile()).unwrap()).unwrap();
        let items: &[Config] = &config["config"];
        print!("\nProjects configured with cppm: \n");
        for i in items {
            println!("{}: {}", i.name, i.location);
        }
        //println!("{:?}", items[0].name);
    }
}

pub struct Cppm {
    project_name: String,
    editor: String,
}

impl Cppm {
    fn init() -> Cppm {
        Cppm {
            project_name: String::new(),
            editor: String::new(),
        }
    }
    // note: possibly integrate git_init
    pub fn spawn(_project_name: String, editor: String, init_type: &str) {
        if Path::new(&_project_name).exists() {
            println!("{}", "Folder with the same name already exists!".red());
            process::exit(0);
        }

        if init_type == "c" {
            let mut s = Cppm::init();
            s.project_name = _project_name;
            let pn = s.project_name.clone();
            s.editor = editor;
            fs::create_dir_all(s.project_name.clone()).expect("Folder creation failed.");
            fs::create_dir_all(format!("{}/src", s.project_name)).expect("Folder creation failed.");

            if !s.editor.contains("null") {
                let mut child = if cfg!(target_os = "windows") {
                    Command::new("powershell")
                        .arg(&format!("{} {}", s.editor, pn))
                        .spawn()
                        .expect("Failed to open editor.")
                } else if cfg!(target_os = "linux") || cfg!(target_os = "unix") {
                    Command::new("sh")
                        .arg(&format!("{} {}", s.editor, pn))
                        .spawn()
                        .expect("Failed to open editor.")
                } else {
                    println!(
                        "{}",
                        "Your OS is not supported, please open an issue to get it implemented."
                            .red()
                    );
                    return;
                };
                child.wait().expect("Failed to wait on process.");
            }
            let main = c_path(s);
            let main_path = Path::new(main.as_str());

            File::create(&main_path)
                .expect("File creation failed.")
                .write_all(misc::CBOILER.as_bytes())
                .expect("Failed to write to main file.");

            Cppm::cppm_toml(
                &format!("{}/{}", std::env::current_dir().unwrap().display(), pn)
                    .replace('\\', "/"),
            );
            write(
                pn.as_str(),
                &format!("{}/{}", std::env::current_dir().unwrap().display(), pn),
            );
        } else {
            let mut s = Cppm::init();
            s.project_name = _project_name;
            let pn = s.project_name.clone();
            s.editor = editor;
            fs::create_dir_all(s.project_name.clone()).expect("Folder creation failed.");
            fs::create_dir_all(format!("{}/src", s.project_name)).expect("Folder creation failed.");
            fs::create_dir_all(format!("{}/include", s.project_name))
                .expect("Folder creation failed.");

            if !s.editor.contains("null") {
                let mut child = if cfg!(target_os = "windows") {
                    Command::new("powershell")
                        .arg(&format!("{} {}", s.editor, pn))
                        .spawn()
                        .expect("Failed to open editor.")
                } else if cfg!(target_os = "linux") || cfg!(target_os = "unix") {
                    Command::new("sh")
                        .arg(&format!("{} {}", s.editor, pn))
                        .spawn()
                        .expect("Failed to open editor.")
                } else {
                    println!(
                        "{}",
                        "Your OS is not supported, please open an issue to get it implemented."
                            .red()
                    );
                    return;
                };
                child.wait().expect("Failed to wait on process.");
            }
            let (main, header) = path(s);
            let main_path = Path::new(main.as_str());
            let header_path = Path::new(header.as_str());

            File::create(&main_path)
                .expect("File creation failed.")
                .write_all(misc::CPPBOILER.as_bytes())
                .expect("Failed to write to main file.");
            File::create(&header_path)
                .expect("File creation failed.")
                .write_all(misc::header_boiler(pn.as_str()).as_bytes())
                .expect("failed to write to header file.");

            Cppm::cppm_toml(
                &format!("{}/{}", std::env::current_dir().unwrap().display(), pn)
                    .replace('\\', "/"),
            );
            write(
                pn.as_str(),
                &format!("{}/{}", std::env::current_dir().unwrap().display(), pn),
            );
        }
    }

    /// note: add aliases for known editors
    pub fn open(_project_name: String, editor: String) {
        if !Path::new(&misc::configfile()).exists() {
            println!("{}", "You have not created any projects yet!".red());
            process::exit(0);
        }
        if builder::subprocess(&editor, "").is_err() {
            println!(
                "    {}",
                "Editor does not exist or cannot be opened with the argument passed.".red()
            );
            process::exit(0);
        }
        let toml_config: HashMap<String, Vec<Config>> =
            toml::from_str(&std::fs::read_to_string(misc::configfile()).unwrap()).unwrap();
        let config: &[Config] = &toml_config["config"];
        let mut b = false;
        for i in config {
            if i.name == _project_name {
                let project_location = i.location.clone();
                println!(
                    "   Opening Project `{}`: {}",
                    _project_name.green(),
                    project_location
                );

                let mut editor = if cfg!(target_os = "windows") {
                    Command::new("powershell")
                        .args(["/c", &format!("{} {}", editor, project_location)])
                        .spawn()
                        .expect("Failed to open editor.")
                } else if cfg!(target_os = "linux") || cfg!(target_os = "unix") {
                    Command::new("sh")
                        .args(["-c", &format!("{} {}", editor, project_location)])
                        .spawn()
                        .expect("Failed to open editor.")
                } else {
                    println!(
                        "{}",
                        "Your OS is not supported, please open an issue to get it implemented."
                            .red()
                    );
                    return;
                };
                b = true;
                editor.wait().expect("Failed to wait on process.");
            }
        }
        if !b {
            println!(
                "    {}",
                "Project does not exist or was not created with cppm!".red()
            );
        }
    }

    pub fn clean() {
        if !Path::new("build").exists() {
            println!("{}", "Project has not been built!".red());
            process::exit(0);
        } else {
            fs::remove_dir_all("build").ok();
        }
    }

    pub fn watch(file: String) {
        let mut original_contents = fs::read_to_string(&file)
            .expect("That file either doesn't exist or cannot be accessed!");

        loop {
            let contents = fs::read_to_string(&file)
                .expect("That file either doesn't exist or cannot be accessed!");

            if contents != original_contents && contents != "" {
                // `contents != ""` is a fix for a bug with VSC - it does not impair normal usage
                original_contents = contents;
                run(false, true);
            }
        }
    }

    pub fn status() {
        let current_version = Command::new("cppm")
            .arg("-V")
            .output()
            .expect("An internal error occurred - please contact a developer");

        let current_version_str = match str::from_utf8(&current_version.stdout) {
            Ok(value) => value.trim(),
            Err(..) => panic!("An internal error occurred - please contact a developer"),
        };

        let result = ureq::get(
            "https://api.github.com/repos/maou-shimazu/cpp-project-manager/releases/latest",
        )
        .call()
        .unwrap()
        .into_string()
        .unwrap();
        let json_value: Value = serde_json::from_str(&result).unwrap();

        let latest = &json_value["tag_name"];

        println!(
            "Current version: {} - Latest version: cppm {}",
            current_version_str, latest
        )
    }

    pub fn init_existing(name: String, mut repo: String) {
        // TODO: Add nice error messages
        repo = format!("https://github.com/{name}/{repo}.git");
        Command::new("git")
            .arg("init")
            .output()
            .expect("An error initializing Git - Make sure you have Git installed and try again!");
        Command::new("git")
            .arg("add")
            .arg("-A")
            .output()
            .expect("An error occurred while trying to commit changes to Git - Make sure you have Git installed and try again");
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("Initial commit")
            .output()
            .expect("An error occured while trying to set the branch to main - Make sure you have Git installed and try again");
        Command::new("git")
            .arg("branch")
            .arg("-M")
            .arg("main")
            .output()
            .expect("An error occured while trying to set the branch to main - Make sure you have Git installed and try again");
        Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(repo)
            .output()
            .expect("An error occurred while trying to connect to the remote repository - Make sure the repository exists and try again");
        Command::new("git")
            .arg("push")
            .arg("-u")
            .arg("origin")
            .arg("main")
            .output()
            .expect("An error occurred while trying to push changes to the repository - Make sure the repository exists and try again");
    }

    /// Initializes a project in the current directory.
    pub fn initialize(init_type: &str) -> std::io::Result<()> {
        if Path::new("src").exists() || Path::new("include").exists() {
            println!(
                "    {}",
                "You already have a project in this directory!".red()
            );
            process::exit(0);
        }
        if init_type == "c" {
            fs::create_dir_all("src").expect("Folder creation failed or folder already exists.");
            fs::create_dir_all("include")
                .expect("Folder creation failed or folder already exists.");
            File::create("src/main.c")
                .expect("Folder creation failed or folder already exists.")
                .write_all(misc::CBOILER.as_bytes())
                .expect("Unable to write to file.");
            File::create("include/main.h")
                .expect("Unable to create file or file already exists.")
                .write_all(misc::header_boiler_c("main").as_bytes())
                .expect("Unable to write to file.");

            write(
                misc::dir_name().as_str(),
                std::env::current_dir()?.as_os_str().to_str().unwrap(),
            );
            Cppm::cppm_toml(std::env::current_dir()?.as_os_str().to_str().unwrap());
            Ok(())
        } else {
            fs::create_dir_all("src").expect("Folder creation failed or folder already exists.");
            fs::create_dir_all("include")
                .expect("Folder creation failed or folder already exists.");
            File::create("include/main.hpp")
                .expect("Unable to create file or file already exists.")
                .write_all(misc::header_boiler("main").as_bytes())
                .expect("Unable to write to file.");
            File::create("src/main.cpp")
                .expect("Folder creation failed or folder already exists.")
                .write_all(misc::CPPBOILER.as_bytes())
                .expect("Unable to write to file.");

            write(
                misc::dir_name().as_str(),
                std::env::current_dir()?.as_os_str().to_str().unwrap(),
            );
            Cppm::cppm_toml(std::env::current_dir()?.as_os_str().to_str().unwrap());
            Ok(())
        }
    }
    // note: impliment libs
    pub fn cppm_toml(loc: &str) {
        let __loc__ = std::path::Path::new(loc)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let cc = format!(
            r#"[project]
name = "{}"
version = "1.0.0"
edition = "2022"
include = "include"
src = "src/main.cpp"
standard = "11"
"#,
            __loc__
        );
        file::write_file(&format!("{}/Cppm.toml", loc), cc.as_bytes())
            .expect("Unable to write to file.");
    }
}

fn path(s: Cppm) -> (String, String) {
    let main: String = format!("{}/src/main.cpp", s.project_name);
    let header: String = format!("{0}/include/{0}.hpp", s.project_name);
    (main, header)
}

// note: find a way to impliment removing from the config file
pub fn remove(project_name: String) {
    let toml_config: HashMap<String, Vec<Config>> =
        toml::from_str(&fs::read_to_string(misc::configfile()).unwrap()).unwrap();
    let config: &[Config] = &toml_config["config"];
    let project = config.iter().find(|p| p.name == project_name);
    if project.is_none() {
        println!("Project does not exist or was not created with cppm!");
        process::exit(1);
    }
    let project_location = project.unwrap().location.clone();
    println!(
        "   Removing Project `{}`: {}",
        project_name.green(),
        project_location
    );
    fs::remove_dir_all(project_location).expect("Failed to remove project.");
    process::exit(0);
}

fn c_path(s: Cppm) -> String {
    let main: String = format!("{}/src/main.cpp", s.project_name);
    main
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Def {
    compilers: HashMap<String, String>,
}
impl Def {
    pub fn new() -> Def {
        Def {
            compilers: HashMap::new(),
        }
    }
}

pub fn defaults_file() -> String {
    let defaultsdir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        .replace('"', "")
        .replace('\\', "/");
    format!("{}/.cppm/defaults.toml", defaultsdir)
}
pub fn defaults() {
    let mut config: Def = Def::new();
    file::ensure_exists(&defaults_file()).ok();
    let c = builder::c();
    let cpp = builder::cpp();

    match c {
        Ok(x) => match x {
            builder::Compilers::Clang => {
                config
                    .compilers
                    .insert("c".to_string(), "clang".to_string());
            }
            builder::Compilers::Gcc => {
                config.compilers.insert("c".to_string(), "gcc".to_string());
            }
            _ => (),
        },
        Err(e) => println!("{}", e),
    }

    match cpp {
        Ok(x) => match x {
            builder::Compilers::Clangpp => {
                config
                    .compilers
                    .insert("cpp".to_string(), "clang++".to_string());
            }
            builder::Compilers::Gpp => {
                config
                    .compilers
                    .insert("cpp".to_string(), "g++".to_string());
            }
            _ => (),
        },
        Err(e) => println!("{}", e),
    }

    file::write_file(
        &defaults_file(),
        toml::to_string(&config).unwrap().as_bytes(),
    )
    .expect("Unable to write to file.");

    println!("Location: {}", defaults_file());
}

// warning: file dosent spawn properly
pub fn toml() {
    println!("location: {}", misc::configfile());
    Command::new(misc::configfile())
        .spawn()
        .expect("Couldn't start editor.");
}

pub fn git_init() {
    Command::new("git")
        .arg("init")
        .stdout(process::Stdio::null())
        .spawn()
        .expect("Couldn't start git.");
}
pub fn git_exists() -> bool {
    builder::subprocess("git", "--version").is_ok()
}
