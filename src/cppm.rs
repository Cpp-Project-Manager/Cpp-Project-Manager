use colored::Colorize;
use fsio::file;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;
use std::process::Command;
#[allow(unused_imports)]
use walkdir::WalkDir;
mod builder;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    location: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LocalConfig {
    project: HashMap<String, String>,
}
pub fn write(project_name: &str, location: &str) {
    file::ensure_exists(&misc::configfile()).ok();

    let config: Config = Config {
        name: project_name.to_string(),
        location: location.replace("/", "\\"),
    };
    fsio::file::ensure_exists(&misc::configfile()).ok();
    file::append_file(
        &misc::configfile(),
        format!("\n[[config]]\n{}", toml::to_string_pretty(&config).unwrap()).as_bytes(),
    )
    .expect("config not written to.");
}

pub mod misc {
    use crate::cppm::Config;
    use std::collections::HashMap;

    pub const CBOILER: &str = r#"
#include <stdio.h>

int main(void) {
    printf("Hello, cppm!\n");
    return 0;
}
"#;

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
            header_name.to_uppercase()
        )
    }

    pub fn configfile() -> String {
        let configdir = dirs::config_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
            .replace('"', "")
            .replace("\\", "/");
        format!("{}/cppm/config.toml", configdir)
    }

    pub fn version() -> String {
        "cppm 3.0.0 (5-11-2022)".to_string() // Warning: update date
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

    pub fn spawn(_project_name: String, editor: String, init_type: &str) {
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
                        "Your OS is not supported, please open an issue to get it implemented.".red()
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
    
            Cppm::cppm_ini(
                &format!("{}/{}", std::env::current_dir().unwrap().display(), pn).replace("\\", "/"),
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
            fs::create_dir_all(format!("{}/include", s.project_name)).expect("Folder creation failed.");
    
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
                        "Your OS is not supported, please open an issue to get it implemented.".red()
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
    
            Cppm::cppm_ini(
                &format!("{}/{}", std::env::current_dir().unwrap().display(), pn).replace("\\", "/"),
            );
            write(
                pn.as_str(),
                &format!("{}/{}", std::env::current_dir().unwrap().display(), pn),
            );
        }
    }

    /// note: add aliases for known editors
    pub fn open(_project_name: String, editor: String) {
        // Note: Make sure to include the if statement below for all commands that require you to do something with a project!
        if !Path::new(&misc::configfile()).exists() {
            println!("{}", "You have not created any projects yet!".red());
            process::exit(0);
        }
        let toml_config: HashMap<String, Vec<Config>> =
            toml::from_str(&std::fs::read_to_string(misc::configfile()).unwrap()).unwrap();
        let config: &[Config] = &toml_config["config"];
        
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
                        "Your OS is not supported, please open an issue to get it implemented.".red()
                    );
                    return;
                };
                editor.wait().expect("Failed to wait on process.");
            } else {
                //println!("Project does not exist or was not created with cppm!"); // note: add appropriate fix for this
            }
        }
    }

    pub fn clean() {
        let build: &str = &format!("{}/build", std::env::current_dir().unwrap().to_str().unwrap());
        if !Path::new(build).exists() {
            println!("{}", "Project has not been built!".red());
            process::exit(0);
        }
        else {
            fs::remove_dir_all(build).ok();
        }
    }

    /// initializes a project in the current directory.
    pub fn initialize(init_type: &str) -> std::io::Result<()> {
        if init_type == "c" {
            fs::create_dir_all("src").expect("Folder creation failed or folder already exists.");
            File::create("src/main.c")
                .expect("Folder creation failed or folder already exists.")
                .write_all(misc::CBOILER.as_bytes())
                .expect("Unable to write to file.");
    
            write(
                misc::dir_name().as_str(),
                &std::env::current_dir()?
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
            Cppm::cppm_ini(std::env::current_dir()?.as_os_str().to_str().unwrap());
            Ok(())
        } else {
            fs::create_dir_all("src").expect("Folder creation failed or folder already exists.");
            fs::create_dir_all("include").expect("Folder creation failed or folder already exists.");
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
                &std::env::current_dir()?
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
            Cppm::cppm_ini(std::env::current_dir()?.as_os_str().to_str().unwrap());
            Ok(())
        }
    }
    pub fn cppm_ini(loc: &str) {
        let __loc__ = std::path::Path::new(loc)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let config: LocalConfig = LocalConfig {
            project: HashMap::from([
                ("name".to_owned(), __loc__),
                ("version".to_owned(), "1.0.1".to_owned()),
                ("edition".to_owned(), "2021".to_owned()),
                ("include".to_owned(), "".to_owned()),
                ("src".to_owned(), "".to_owned()),
            ]),
        };
        file::write_file(
            &format!("{}/Cppm.toml", loc),
            toml::to_string(&config).unwrap().as_bytes(),
        )
        .expect("Unable to write to file.");
    }
}

fn path(s: Cppm) -> (String, String) {
    let main: String = format!("{}/src/main.cpp", s.project_name);
    let header: String = format!("{0}/include/{0}.hpp", s.project_name);
    (main, header)
}

pub fn remove(project_name: String) {
    let toml_config: HashMap<String, Vec<Config>> =
        toml::from_str(&std::fs::read_to_string(misc::configfile()).unwrap()).unwrap();
    let config: &[Config] = &toml_config["config"]; // config is a vector of Config structs
    
    for i in config {
        if i.name == project_name {
            let project_location = i.location.clone();
            println!(
                "   Removing Project `{}`: {}",
                project_name.green(),
                project_location
            );
            fs::remove_dir_all(project_location).expect("Failed to remove project.");
            // find a way to remove from file
            process::exit(0);
        }   
    }
    println!("Project does not exist or was not created with cppm!");
}

fn c_path(s: Cppm) -> String {
    let main: String = format!("{}/src/main.cpp", s.project_name);
    main
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Def {
    editor: String,
    compilers: HashMap<String, String>,
}
impl Def {
    pub fn new() -> Def {
        Def {
            editor: String::new(),
            compilers: HashMap::new(),
        }
    }
}

pub fn defaults_file() -> String {
    let defaultsdir = dirs::config_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        .replace('"', "")
        .replace("\\", "/");
    format!("{}/cppm/defaults.toml", defaultsdir)
}
pub fn defaults() {
    let mut config: Def = Def::new();
    let mut ans: String = String::new();
    file::ensure_exists(&defaults_file()).ok();
    print!("Default Editor: ");
    use std::io::stdout;
    stdout().flush().ok();
    std::io::stdin().read_line(&mut ans).ok();

    config.editor = ans.trim().to_string();
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
