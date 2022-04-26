//use std::env;
use std::fs;
use std::process::Command;

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
        s.editor = editor;
        println!("Editor: {}", s.editor);
        fs::create_dir_all(s.project_name).expect("folder creation failed.");
        if !s.editor.contains("null") {
            let mut _process = if cfg!(target_os = "windows") {
                Command::new("powershell")
                    .args(["/c", format!("{}", s.editor).as_str(), "."])
                    .output()
                    .expect("failed to execute process");
            } else {
                Command::new("sh")
                    .args(["-c", format!("{}", s.editor).as_str(), "."])
                    .output()
                    .expect("failed to execute process");
            };
        }
    }
}
