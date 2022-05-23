use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process;
use fsio::file;
use walkdir::WalkDir;
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    location: String,
}

impl Config {
    fn new(name: String, location: String) -> Config {
        Config { name, location }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    version: String,
    edition: String,
    include: String,
    src: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LocalConfig {
    project: Project,
}

pub fn write(project_name: &str, location: &str) {
    
    file::ensure_exists(&misc::configfile()).ok();
    let config = Config {
        name: project_name.to_string(),
        location: location.to_string(),
    };
    fsio::file::ensure_exists(&misc::configfile()).ok();
    let conf: String = misc::configfile();
    let file = std::fs::read_to_string(misc::configfile()).unwrap();
    
    if file.contains(&config.name) {
        println!("{}", "Error: Project already exists.".red());
        std::process::exit(0);
    } else {
        file::append_file(
            conf.as_str(),
            toml::to_string_pretty(&config).unwrap().as_bytes(),
        )
        .expect(&format!("{}", "Error: Config file not written to.".red()));
    }
    
}

pub mod misc {
    use crate::cppm::Config;

    pub const CPPBOILER: &str = r#"#include <iostream>

int main(){

    std::cout << "Hello, cppm!" << std::endl;
    return 0;
}
"#;

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
        "cppm 3.0.0 (5-11-2022)".to_string() // NOTICE: Update version / date accordingly
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
        let config: Config =
            toml::from_str(&std::fs::read_to_string(configfile()).unwrap()).unwrap();
        print!("\nProjects configured with cppm: \n");
        println!("{}", config.name);
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

    pub fn spawn(_project_name: String, editor: String) {
        let mut s = Cppm::init();
        s.project_name = _project_name;
        let pn = s.project_name.clone();
        s.editor = editor;
        fs::create_dir_all(s.project_name.clone()).expect(&format!("{}", "Error: Folder creation failed".red()));
        fs::create_dir_all(format!("{}/src", s.project_name)).expect(&format!("{}", "Error: Folder creation failed".red());
        fs::create_dir_all(format!("{}/include", s.project_name)).expect(&format!("{}", "Error: Folder creation failed".red());

        if !s.editor.contains("null") {
            let mut child = if cfg!(target_os = "windows") {
                
                Command::new("powershell")
                    .arg(&format!("{} {}", s.editor, pn))
                    .spawn()
                    .expect(&format!("{}", "Error: Failed to open editor".red())
                
            } else if cfg!(target_os = "linux") || cfg!(target_os = "unix") {
                
                Command::new("sh")
                    .arg(&format!("{} {}", s.editor, pn))
                    .spawn()
                    .expect(&format!("{}", "Error: Failed to open editor".red())
                
            } else {
                println!(
                    "{}",
                    "Error: Your OS is not supported, please open an issue to get it implemented.".red()
                );
                return;
            };
            child.wait().expect(&format!("{}", "Error: Failed to wait on process".red());
        }
        let (main, header) = path(s);
        let main_path = Path::new(main.as_str());
        let header_path = Path::new(header.as_str());

        File::create(&main_path)
            .expect(&format!("{}", "Error: File creation failed".red())
            .write_all(misc::CPPBOILER.as_bytes())
            .expect(&format!("{}", "Error: Failed to write to main file".red());
        File::create(&header_path)
            .expect(&format!("{}", "Error: File creation failed".red())
            .write_all(misc::header_boiler(pn.as_str()).as_bytes())
            .expect(&format!("{}", "Error: Failed to write to header file".red());

        Cppm::cppm_ini(
            &format!("{}/{}", std::env::current_dir().unwrap().display(), pn).replace("\\", "/"),
        );
        write(
            pn.as_str(),
            &format!("{}/{}", std::env::current_dir().unwrap().display(), pn),
        );
    }
                
    pub fn open(_project_name: String, editor: String) {
        // Notice: Make sure to include the if statement below for all commands that require you to do something with a project!
        if Path::new(&misc::configfile()).exists() == false {
            println!("{}", "You have not created any projects yet!".red());
            process::exit(0);
        }
        let t: Config =
            toml::from_str(&std::fs::read_to_string(misc::configfile()).unwrap()).unwrap();
        let key = format!("project.{}", _project_name);
        if t.name == key {
            let project_location = t.location;
            println!(
                "Opening Project{}`: {}",
                key.replace("project.", " `").green(),
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
                    "Error: Your OS is not supported, please open an issue to get it implemented.".red()
                );
                return;
            };
            editor.wait().expect(&format!("{}", "Error: Failed to wait on proccess".red());
        } else {
            println!("Error: Project does not exist or was not created with cppm!".red());
        }
    }

    pub fn clean(project_name: &str) {
        // Notice: Make sure to include the if statement below for all commands that require you to do something with a project!
        if Path::new(&misc::configfile()).exists() == false {
            println!("{}", "Error: You have not created any projects yet!".red());
            process::exit(0);
        }
        let t: Config =
        toml::from_str(&std::fs::read_to_string(misc::configfile()).unwrap()).unwrap();
        let project_location = t.location;
        fs::remove_dir_all(&project_location);
    }

    pub fn initialize() -> std::io::Result<()> {
        fs::create_dir_all("src").expect(&format!("{}", "Error: Folder creation failed or folder already exists".red());
        fs::create_dir_all("include").expect(&format!("{}", "Error: Folder creation failed or folder already exists".red());
        File::create("include/main.hpp")
            .expect(&format!("{}", "Error: File creation failed or file already exists".red())
            .write_all(misc::header_boiler("main").as_bytes())
            .expect(&format!("{}", "Error: Unable to write to file".red());
        File::create("src/main.cpp")
            .expect(&format!("{}", "Error: Folder creation failed or folder already exists".red())
            .write_all(misc::CPPBOILER.as_bytes())
            .expect(&format!("{}", "Error: Unable to write to file".red());

        write(
            misc::dir_name().as_str(),
            std::env::current_dir()?.as_os_str().to_str().unwrap(),
        );
        Cppm::cppm_ini(std::env::current_dir()?.as_os_str().to_str().unwrap());
        Ok(())
    }
        
    pub fn cppm_ini(loc: &str) {
        let __loc__ = std::path::Path::new(loc)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let project: Project = toml::from_str(&format!(
            "name='{}'\n version='1.0.1'\n edition='2021' \ninclude=''\nsrc=''\n",
            __loc__,
        ))
        .unwrap();
        let config: LocalConfig = LocalConfig { project };
        println!("{}", loc);
        file::write_file(
            &format!("{}/Cppm.toml", loc),
            toml::to_string(&config).unwrap().as_bytes(),
        )
        .expect(&format!("{}", "Error: Unable to write to file".red());
    }
}
fn path(s: Cppm) -> (String, String) {
    let main: String = format!("{}/src/main.cpp", s.project_name);
    let header: String = format!("{0}/include/{0}.hpp", s.project_name);
    (main, header)
}

static HELLO: &str = "";

pub mod defaults {
    use fsio::file;
    use std::io::{stdout, Write};
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
}
