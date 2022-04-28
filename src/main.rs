mod cppm;
use std::env;
use colored::Colorize;

fn main() {


    let _args: Vec<String> = env::args().collect();
    let _menu_answer: i8;

    //todo: Impliment `cppm open <project_name> <editor>`
    //note: `cppm list projects` is also a possible implimentation.

    match _args.len(){
        1 => {
            println!("No args passed");
        },
        2 | 3 | 4 => {
            match _args[1].as_str() {
                "new" => { // possibly add minimal support for C
                    if _args.len() > 3 {
                        cppm::Cppm::new(_args[2].clone(), _args[3].clone());
                    }
                    else {
                       cppm::Cppm::new(_args[2].clone(), "null".to_string()); 
                    }
                    println!("    {} {} `{}`", "Created".bright_green(), "C++ project" , _args[2]);
                },
                "init" => (),
                "run" => (),
                "build" => (),
                "clean" => (),
                "release" => (),
                "open" => {
                    if _args.len() > 3 {
                        cppm::Cppm::open(_args[2].clone(), _args[3].clone());
                    }
                    else {
                        println!("   {}", "Please provide a text editor.".bright_red())
                    }
                }
                "config" => (),
                "help" | _ => println!("{}", cppm::misc::HELP),
            }
        }
        _ => {
            println!("Too many args");
        }
    }
}
