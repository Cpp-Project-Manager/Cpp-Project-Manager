mod cppm;
use colored::Colorize;
use cppm::*;
use std::env;
use std::process::Command;
const OPTIONS: &str = r#"OPTIONS:
    -h, --help      Displays this help message.
    -v, --version   Displays the version of this program.
    -l, --list      Lists all configured projects.

COMMANDS:
    config          Configures cppm.
    new             Creates a new project.                              [OPTIONAL ARGUMENT: -c (Initializes it with C code)]
    init            Initializes a project in the current directory.     [OPTIONAL ARGUMENT: -c (Initializes it with C code)]
    open            Opens a project that was created with cppm.
    lp              Lists all projects configured with cppm.
    *build          Builds the project to a dist directory.
    *run            Build and Runs the project.
    *clean          Cleans the project dist.
    *remove         Removes a project from configuration.

Note: Many commands have not been implemented yet. This is a minor release, more features will be added in the future. Commands with * are not yet implemented.
"#;

fn man() {
    println!("C++ Project Manager\n");
    println!("USAGE:\n     cppm [COMMANDS] [+SUBCOMMANDS] [+NESTED-SC]\n");
    println!("{}", OPTIONS);
}

fn main() {
    let _args: Vec<String> = env::args().collect();

    // note: `cppm list projects` is also a possible implimentation.
    // note: human panic
    // note: use config to configure defaults as well
    // note: MN: Clint
    match _args.len() {
        1 => {
            man();
        }
        2 | 3 | 4 => {
            match _args[1].as_str() {
                "-v" | "--version" => {
                    println!("{}", misc::version());
                }
                "new" => {
                    if _args.len() == 4 {
                        if _args[3] == "-c" {
                            println!("{:?}", _args);
                            Cppm::spawn(_args[2].clone(), "null".to_string(), "c")
                            println!(
                                "    {} C project `{}`",
                                "Created".bright_green(),
                                _args[2]
                            );
                        } else {
                            println!("Invalid arguments!");
                            return
                        }
                    } else if _args.len() > 3 {
                        Cppm::spawn(_args[2].clone(), _args[3].clone(), "cpp");
                    } else if _args.len() > 2 {
                        println!(
                            "    {} C++ project `{}`",
                            "Created".bright_green(),
                            _args[2]
                        );
                        Cppm::spawn(_args[2].clone(), "null".to_string(), "cpp");
                        println!(
                            "    {} C++ project `{}`",
                            "Created".bright_green(),
                            _args[2]
                        );
                    } else {
                        println!("{}", "Error: You must provide a project name.".red());
                        return;
                    }
                }
                "init" => {
                    if _args.len() == 3 {
                        if _args[2] == "-c" {
                            Cppm::initialize("c").ok();
                        } else {
                            println!("Invalid arguments!")
                        }
                    } else {
                        Cppm::initialize("cpp").ok();
                    }
                }
                "lp" => {
                    misc::list_projects();
                }
                "run" => (),
                "build" => (),
                "clean" => Cppm::clean(),
                "release" => (),
                "remove" => (), //todo:
                "open" => {
                    let editor = env::var("EDITOR").unwrap_or_else(|_| "".to_string());
                    if _args.len() > 3 {
                        Cppm::open(_args[2].clone(), _args[3].clone());
                    } else {
                        if _args.len() == 2 {
                            println!("{}", "Error: You need to provide a project name.".red());
                            return;
                        }
                        if editor.is_empty() {
                            println!("   {}", "Please provide a text editor.".bright_red());
                            return;
                        }
                        Cppm::open(_args[2].clone(), editor);
                    }
                }
                "config" => {
                    cppm::defaults();
                }
                "ini" => {
                    
                    Command::new("notepad")
                        .arg(misc::configfile())
                        .spawn()
                        .expect("Couldn't start notepad.");
                    #[cfg(unix)]
                    Command::new("nvim")
                        .arg(misc::configfile())
                        .spawn()
                        .expect("Couldnt start Nvim.");
                    println!("location: {}", misc::configfile())
                }
                "test" => {}
                "--help" | "-h" => man(),
                _ => man(),
            }
        }
        _ => {
            println!(
                "   {}",
                "Argument not supported, please use `cppm --help` for more info.".bright_red()
            );
        }
    }
}
