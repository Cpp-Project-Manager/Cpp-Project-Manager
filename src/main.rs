mod cppm;
use std::env;

fn main() {


    let _args: Vec<String> = env::args().collect();
    let _menu_answer: i8;

    match _args.len(){
        1 => {
            println!("No args passed");
        },
        2 | 3 | 4 => {
            match _args[1].as_str() {
                "new" => {
                    println!("Creating new project: {}" , _args[2]);
                    if _args.len() > 3 {
                        cppm::Cppm::new(_args[2].clone(), _args[3].clone());
                    }
                    else {
                       cppm::Cppm::new(_args[2].clone(), "null".to_string()); 
                    }
                },
                "run" => {

                },
                "build" => {

                },
                "clean" => {

                }
                "release" => {

                }
                "help" | _ => println!("{}", cppm::misc::HELP),
            }
        }
        _ => {
            println!("Too many args");
        }
    }
}
