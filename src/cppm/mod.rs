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
            .replace('"', "");
        format!("{}/cppm/config.ini", configdir)
    }

    pub fn write(_project_name_section: &str, _location: &str) {
        let mut conf: String = configfile();
        if !file::ensure_exists(conf.as_str()).is_ok() {
            std::fs::File::create(&mut conf).expect("Failed to create config file.");
        }
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
    pub fn version() -> String{
        "cppm 0.2.1 (22-04-28)".to_string()
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

    pub fn new(_project_name: String, editor: String) {
        let mut s = Cppm::init();
        s.project_name = _project_name;
        let pn = s.project_name.clone();
        s.editor = editor;
        fs::create_dir_all(s.project_name.clone()).expect("folder creation failed.");
        fs::create_dir_all(format!("{}/src", s.project_name.clone()))
            .expect("folder creation failed.");
        fs::create_dir_all(format!("{}/include", s.project_name.clone()))
            .expect("folder creation failed.");

        if !s.editor.contains("null") {
            #[cfg(windows)]
            Command::new("powershell")
                .args([
                    "/c",
                    format!("cd {};", s.project_name.clone()).as_str(),
                    format!("{} .", s.editor).as_str(),
                ])
                .output()
                .expect("failed to execute process");
            #[cfg(unix)]
            Command::new("sh")
                .args([
                    "-c",
                    format!("cd {} && ", s.project_name.clone()).as_str(),
                    format!("{} .", s.editor).as_str(),
                ])
                .output()
                .expect("failed to execute process");
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

        misc::write(
            pn.clone().as_str(),
            format!(
                "{}/{}",
                std::env::current_dir().unwrap().display(),
                pn.clone()
            )
            .as_str(),
        );
    }
    // note: add aliases for known editors
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

            #[cfg(windows)]
            Command::new("powershell")
                .args([
                    "/c",
                    format!("cd {}; {} .", project_location, editor).as_str(),
                ])
                .output()
                .expect("couldnt spawn command");
            #[cfg(unix)]
            Command::new("sh")
                .args([
                    "-c",
                    format!("cd {} && {} .", project_location, editor).as_str(),
                ])
                .output()
                .expect("couldnt spawn command");
        } else {
            println!("Project does not exist or was not created with cppm!!");
        }
    }
}
fn path(s: Cppm) -> (String, String) {
    let main: String = format!("{}/src/main.cpp", s.project_name);
    let header: String = format!("{0}/include/{0}.hpp", s.project_name);
    (main, header)
}
