//use std::env;
use std::fs;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub mod misc {
    pub const HELP: &str = r#"
[X] cp create - Creates new project with your specifications.
[X] cp new    - {project} Creates a boiler plate project.
[X] cp help   - Displays this help message.
"#;
    pub const CPPBOILER: &str = 
r#"#include <iostream>

int main(){

    std::cout << "Hello World" << std::endl;
    return 0;            
} 
"#;
    pub fn header_boiler(header_name: &str) -> String {
        format!(
r#"#pragma once

#ifndef {0}
#define {0}

#include <iostream>


#endif"#,
            header_name.to_uppercase().replace(".", "_")
        )
    }
}

pub struct Cppm {
    #[allow(dead_code)]
    ans: i16,
    #[allow(dead_code)]
    project_template_ans: i16,
    #[allow(dead_code)]
    folder_name: String,
    project_name: String,
    #[allow(dead_code)]
    answer: String,
    #[allow(dead_code)]
    folder_create: String,
    #[allow(dead_code)]
    folder_remove: String,
    #[allow(dead_code)]
    folder: String,
    editor: String,
    #[allow(dead_code)]
    class_name: String,
    #[allow(dead_code)]
    path: String,
}

impl Cppm {
    fn init() -> Cppm {
        Cppm {
            ans: 0,
            project_template_ans: 0,
            folder_name: String::new(),
            project_name: String::new(),
            answer: String::new(),
            folder_create: String::new(),
            folder_remove: String::new(),
            folder: String::new(),
            editor: String::new(),
            class_name: String::new(),
            path: String::new(),
        }
    }

    pub fn new(_arg: String, editor: String) {
        let mut s = Cppm::init();
        s.project_name = _arg;
        let pn = s.project_name.clone();
        s.editor = editor;
        println!("Editor: {}", s.editor); //note: Outputs editor
        fs::create_dir_all(s.project_name.clone()).expect("folder creation failed.");

        #[cfg(windows)]
        fs::create_dir_all(format!("{}\\src", s.project_name.clone())).expect("folder creation failed.");
        #[cfg(windows)]
        fs::create_dir_all(format!("{}\\include", s.project_name.clone())).expect("folder creation failed.");

        #[cfg(unix)]
        fs::create_dir_all(format!("{}/src", s.project_name.clone())).expect("folder creation failed.");
        #[cfg(unix)]
        fs::create_dir_all(format!("{}/include", s.project_name.clone())).expect("folder creation failed.");

        if !s.editor.contains("null") {
            let mut _process = if cfg!(target_os = "windows") {
                Command::new("powershell")
                    .args(["/c", format!("cd {};", s.project_name.clone()).as_str(), format!("{} .", s.editor).as_str()]).output().expect("failed to execute process")
            } else {
                Command::new("sh")
                    .args(["-c", format!("cd {} && ", s.project_name.clone()).as_str(), format!("{} .", s.editor).as_str()]).output().expect("failed to execute process")
            };
        }
        let (main, header) = path(s);
        let main_path = Path::new(main.as_str());
        let header_path = Path::new(header.as_str());

        println!("{}, {}", main_path.display(), header_path.display()); //note: outputs files

        File::create(&main_path).expect("file creation failed").write_all(misc::CPPBOILER.as_bytes()).expect("failed to write to main file.");
        File::create(&header_path).expect("file creation failed").write_all(misc::header_boiler(pn.as_str()).as_bytes()).expect("failed to write to main file.");
    }
} // Cppm

    #[cfg(windows)]
    fn path(s: Cppm) -> (String, String){
        let main: String = format!("{}\\src\\main.cpp", s.project_name);
        let header: String = format!("{0}\\include\\{0}.hpp", s.project_name);
        (main, header)
    }
    #[cfg(unix)]
    fn path(s: Cppm) -> (String, String){
        let main = format!("{}/src/main.cpp", s.project_name);
        let header = format!("{0}/include/{0}.hpp", s.project_name);
        (main, header)
    }
