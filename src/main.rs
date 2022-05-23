mod cppm;
mod builder;

use colored::Colorize;
use cppm::*;
use std::env;
use std::process::Command;

const OPTIONS: &str = r#"OPTIONS:
    -h, --help      Displays this help message.
    -v, --version   Displays the version of this program.
    -l, --list      Lists all configured projects.

COMMANDS:
    *config         Configures cppm.
    new             Creates a new project.
    init            Initializes a project in the current directory.
    open            Opens a project that was created with cppm.
    lp              Lists all projects configured with cppm.
    clean           Cleans the project dist.
    *build          Builds the project to a dist directory.
    *run            Build and Runs the project.
    *remove         Removes a project from configuration.

Note: Many commands have not been implemented yet. This is a minor release, more features will be added in the future. Commands with * are not yet implemented.
"#;

fn man() {
    println!("C++ Project Manager\n");
    println!("USAGE:\n cppm [COMMANDS] [+SUBCOMMANDS] [+NESTED-SC]\n");
    println!("{}", OPTIONS);
}

fn main() {
    let _args: Vec<String> = env::args().collect();

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
                    if _args.len() > 3 {
                        Cppm::spawn(_args[2].clone(), _args[3].clone());
                    } else if _args.len() > 2 {
                        Cppm::spawn(_args[2].clone(), "null".to_string());
                    } else {
                        println!("{}", "Error: You must provide a project name.".red());
                        return;
                    }
                    println!(
                        "{} C++ project `{}`",
                        "Project created".bright_green(),
                        _args[2]
                    );
                }
                
                "init" => {
                    Cppm::initialize().ok();
                }
                
                "lp" => {
                    misc::list_projects();
                }
                
                "clean" => {
                    if _args.len() == 3 {
                        Cppm::clean(&_args[1]);
                    } else {
                        println!("{}", "Error: Invalid arguments".red())
                    }
                },
                

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
                            println!("{}", "Error: Please provide a text editor.".red());
                            return;
                        }
                        Cppm::open(_args[2].clone(), editor);
                    }
                }     
 
                "ini" => {
                    #[cfg(windows)]
                    Command::new("notepad")
                        .arg(misc::configfile())
                        .spawn()
                        .expect(&format!("{}", "Error: Couldn't start Notepad".red());
                    #[cfg(unix)]
                    Command::new("nvim")
                        .arg(misc::configfile())
                        .spawn()
                        .expect(&format!("{}", "Error: Couldn't start NVim".red());
                }
                
                "config" => {
                    builder::compiler_check();
                }
                
                "--help" | "-h" => man(),                
                
                "run" => (),
                "build" => (),
                "release" => (),
                "remove" => (),
                "test" => (),
                
                _ => {
                    println!("{}", "Error: Not a valid command! Run `cppm -h` to view the man page.".red())
                }

            }
        }
        _ => {
            println!(
                "   {}",
                "Error: Argument not supported, please use `cppm --help` for more info.".red()
            );
        }
    }
}
