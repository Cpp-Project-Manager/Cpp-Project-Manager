use colored::Colorize;
use configparser::ini::Ini;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub mod misc {
    use configparser::ini::Ini;
    use fsio::file;

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
        format!("{}/cppm/config.ini", configdir)
    }
    
    /// Writes to config.ini
    pub fn write(_project_name_section: &str, _location: &str) {
        #[allow(unused_mut)]
        let mut conf: String = configfile();
        // if !file::ensure_exists(conf.as_str()).is_ok() {
        //     std::fs::File::create(&mut conf).expect("Failed to create config file.");
        // }
        file::ensure_exists(&configfile()).ok();
        let mut ini = Ini::new();
        let mut temp_ini = Ini::new();
        let sec: String = format!("project.{}", _project_name_section);
        let check = temp_ini.load(conf.clone()).unwrap();
        ini.set(
            sec.as_str(),
            "location",
            Some(_location.replace("\\", "/").to_owned()),
        );
        if check.contains_key(sec.as_str()) {
            temp_ini.write(conf).expect("config not written to");
        } else {
            file::append_file(conf.as_str(), ini.writes().as_bytes())
                .expect("config not written to.");
        }
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
    // note: put on gists
    /// Shows all the projects in config.ini
    pub fn list_projects() {
        let map = Ini::new().load(configfile()).unwrap();
        print!("\nProjects configured with cppm: \n");
        map.iter().for_each(|(x, y)| {
            print!("{}: ", x.replace("project.", ""));
            y.iter().for_each(|(_, b)| {
                println!("{}", b.clone().unwrap());
            });
        });
    }
}

pub struct Cppm {
    project_name: String,
    editor: String,
}

impl Cppm {
    fn __init__() -> Cppm {
        Cppm {
            project_name: String::new(),
            editor: String::new(),
        }
    }

    pub fn new(_project_name: String, editor: String) {
        let mut s = Cppm::__init__();
        s.project_name = _project_name;
        let pn = s.project_name.clone();
        s.editor = editor;
        fs::create_dir_all(s.project_name.clone()).expect("folder creation failed.");
        fs::create_dir_all(format!("{}/src", s.project_name.clone()))
            .expect("folder creation failed.");
        fs::create_dir_all(format!("{}/include", s.project_name.clone()))
            .expect("folder creation failed.");

        if !s.editor.contains("null") {
            let mut child = if cfg!(target_os = "windows") {
                Command::new("powershell")
                    .args(["/c", &format!("{} {}", s.editor.clone(), pn)])
                    .spawn()
                    .expect("failed to open editor")
            } else if cfg!(target_os = "linux") || cfg!(target_os = "unix") {
                Command::new("sh")
                    .args(["-c", &format!("{} {}", s.editor.clone(), pn)])
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
            &format!(
                "{}/{}",
                std::env::current_dir().unwrap().display(),
                pn.clone()
            )
            .replace("\\", "/"),
        );

        misc::write(
            pn.clone().as_str(),
            &format!(
                "{}/{}",
                std::env::current_dir().unwrap().display(),
                pn.clone()
            ),
        );
    }
    /// note: add aliases for known editors
    pub fn open(_project_name: String, editor: String) {
        let config_loc = misc::configfile();
        let mut config = Ini::new();
        let ini = config.load(config_loc.clone()).unwrap();
        let key = format!("project.{}", _project_name.clone());
        if ini.contains_key(key.as_str()) {
            let project_location = config.get(key.as_str(), "location").unwrap();
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
    pub fn init() -> std::io::Result<()> {
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

        misc::write(
            misc::dir_name().as_str(),
            &std::env::current_dir()?.as_os_str().to_str().unwrap(),
        );
        Cppm::cppm_ini(&std::env::current_dir()?.as_os_str().to_str().unwrap());
        Ok(())
    }
    pub fn cppm_ini(loc: &str) {
        let mut config = Ini::new();
        println!("{}", loc);
        let __loc__ = Some(
            std::path::Path::new(loc)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
        //File::create(format!("{}/Cppm.ini", loc)).expect("Unable to create file  or already exists.");
        config.set("project", "name", __loc__);
        config.set("project", "version", Some("0.1.0".to_string()));
        config.set("project", "edition", Some("2022".to_string()));
        config.write(format!("{}/Cppm.ini", loc)).ok();
    }
}
fn path(s: Cppm) -> (String, String) {
    let main: String = format!("{}/src/main.cpp", s.project_name);
    let header: String = format!("{0}/include/{0}.hpp", s.project_name);
    (main, header)
}

#[allow(dead_code)]
pub mod defaults {
use fsio::file;
use configparser::ini::Ini;
use std::io::{stdout, Write};
    pub fn defaults_file() -> String {
        let defaultsdir = dirs::config_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
            .replace('"', "")
            .replace("\\", "/");
        format!("{}/cppm/defaults.ini", defaultsdir)
    }
    pub fn defaults() {
        file::ensure_exists(&defaults_file()).ok();
        let mut ini = Ini::new();
        let mut ans: String = String::new();
        print!("Default editor: ");
        stdout().flush().ok();
        std::io::stdin().read_line(&mut ans).ok();
        ini.set("defaults", "editor", Some(ans.clone()));
        ini.write(defaults_file()).expect("Could not write to default configuration file.");
    }

}