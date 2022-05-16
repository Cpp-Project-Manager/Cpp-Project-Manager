use colored::Colorize;
use fsio::file;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

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
    println!("{}", conf);
    file::append_file(
        conf.as_str(),
        toml::to_string_pretty(&config).unwrap().as_bytes(),
    )
    .expect("config not written to.");
}

pub mod misc {
    use crate::cppm::Config;

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
        "cppm 3.0.0 (5-11-2022)".to_string() //warning: update date
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
        fs::create_dir_all(s.project_name.clone()).expect("folder creation failed.");
        fs::create_dir_all(format!("{}/src", s.project_name)).expect("folder creation failed.");
        fs::create_dir_all(format!("{}/include", s.project_name)).expect("folder creation failed.");

        if !s.editor.contains("null") {
            let mut child = if cfg!(target_os = "windows") {
                Command::new("powershell")
                    .arg(&format!("{} {}", s.editor, pn))
                    .spawn()
                    .expect("failed to open editor")
            } else if cfg!(target_os = "linux") || cfg!(target_os = "unix") {
                Command::new("sh")
                    .arg(&format!("{} {}", s.editor, pn))
                    .spawn()
                    .expect("failed to open editor")
            } else {
                println!(
                    "{}",
                    "Your OS is not supported, Please make an issue to get it implemented.".red()
                );
                return;
            };
            child.wait().expect("failed to wait on process");
        }
        let (main, header) = path(s);
        let main_path = Path::new(main.as_str());
        let header_path = Path::new(header.as_str());

        File::create(&main_path)
            .expect("file creation failed")
            .write_all(misc::CPPBOILER.as_bytes())
            .expect("failed to write to main file.");
        File::create(&header_path)
            .expect("file creation failed")
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
    /// note: add aliases for known editors
    pub fn open(_project_name: String, editor: String) {
        //let config_loc = misc::configfile();
        let t: Config =
            toml::from_str(&std::fs::read_to_string(misc::configfile()).unwrap()).unwrap();
        let key = format!("project.{}", _project_name);
        if t.name == key {
            let project_location = t.location;
            println!(
                "   Opening Project{}`: {}",
                key.replace("project.", " `").green(),
                project_location
            );

            let mut editor = if cfg!(target_os = "windows") {
                Command::new("powershell")
                    .args(["/c", &format!("{} {}", editor, project_location)])
                    .spawn()
                    .expect("failed to open editor")
            } else if cfg!(target_os = "linux") || cfg!(target_os = "unix") {
                Command::new("sh")
                    .args(["-c", &format!("{} {}", editor, project_location)])
                    .spawn()
                    .expect("failed to open editor")
            } else {
                println!(
                    "{}",
                    "Your OS is not supported, Please make an issue to get it implemented.".red()
                );
                return;
            };
            editor.wait().expect("failed to wait on process");
        } else {
            println!("Project does not exist or was not created with cppm!!");
        }
    }
    /// initializes a project in the current directory.
    pub fn initialize() -> std::io::Result<()> {
        fs::create_dir_all("src").expect("folder creation failed or already exists.");
        fs::create_dir_all("include").expect("folder creation failed or already exists.");
        File::create("include/main.hpp")
            .expect("Unable to create file or already exists.")
            .write_all(misc::header_boiler("main").as_bytes())
            .expect("unable to write to file.");
        File::create("src/main.cpp")
            .expect("Unable to create file  or already exists.")
            .write_all(misc::CPPBOILER.as_bytes())
            .expect("unable to write to file.");

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
            r#"
                name='x'
                version='-1.0.1'
                edition='2021'
                include='include'
            "#
        ))
        .unwrap();
        let config: LocalConfig = LocalConfig { project };
        println!("{}", loc);
        file::write_file(
            &format!("{}/Cppm.toml", loc),
            toml::to_string(&config).unwrap().as_bytes(),
        )
        .expect("Unable to write to file.");

        //File::create(format!("{}/Cppm.ini", loc)).expect("Unable to create file  or already exists.");
        // config.set("project", "name", __loc__);
        // config.set("project", "version", Some("0.1.0".to_string()));
        // config.set("project", "edition", Some("2022".to_string()));
        // config.set("project", "src", Some("".to_string()));
        // config.set("project", "include", Some("".to_string()));
        // config.write(format!("{}/Cppm.ini", loc)).ok();
    }
}
fn path(s: Cppm) -> (String, String) {
    let main: String = format!("{}/src/main.cpp", s.project_name);
    let header: String = format!("{0}/include/{0}.hpp", s.project_name);
    (main, header)
}

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
    // pub fn defaults() { warning: fix
    //     file::ensure_exists(&defaults_file()).ok();
    //     let mut ini = Ini::new();
    //     let mut ans: String = String::new();
    //     print!("Default editor: ");
    //     stdout().flush().ok();
    //     std::io::stdin().read_line(&mut ans).ok();
    //     ini.set("defaults", "editor", Some(ans.clone()));
    //     ini.write(defaults_file())
    //         .expect("Could not write to default configuration file.");
    // }
}
