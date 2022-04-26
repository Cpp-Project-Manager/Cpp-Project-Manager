mod cppm;
use std::env;

fn main() {
    mod misc {
        #[allow(dead_code)]
        pub const HELP: &str = r#"
[X] cp create - Creates new project with your specifications.
[X] cp new    - {project} Creates a boiler plate project.
[X] cp help   - Displays this help message.
"#;
        #[allow(dead_code)]
        pub const CPPBOILER: &str = r#"
#include <iostream>

int main(){
            
    std::cout << "Hello World" << std::endl;
    return 0;            
})   
"#;
        #[allow(dead_code)]
        pub fn header_boiler(header_name: &str) -> String {
            format!(
                r#"
#pragma once

#ifndef {0}
#define {0}

#include <iostream>


#endif

"#,
                header_name.to_uppercase().replace(".", "_")
            )
        }
    }

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
                "help" | _ => println!("{}", misc::HELP),
            }
        }
        _ => {
            println!("Too many args");
        }
    }
}
