use std::process::*;
 #[allow(dead_code)]
pub enum Compilers{
    GCC,
    CLANG,
    MSVC,
    GPP,
    CLANGPP,
    MAKE,
}

fn subprocess(process: &str, arg: &str)-> std::io::Result<ExitStatus> {
    Command::new(process).arg(arg).stdout(Stdio::null()).stderr(Stdio::null()).status()
}

pub fn compiler_check() -> Vec<Compilers>{
    let mut _e: Vec<Compilers> = Vec::new();
    if subprocess("gcc", "-v").is_ok() {
        println!("{}", "GCC is installed.");
        _e.push(Compilers::GCC);
    }
    if subprocess("g++", "-v").is_ok() {
        println!("{}", "G++ is installed.");
        _e.push(Compilers::GPP);
    }
    if subprocess("clang", "-v").is_ok() {
        println!("{}", "Clang is installed.");
        _e.push(Compilers::CLANG);
    }
    if subprocess("clangpp", "-v").is_ok() {
        println!("{}", "Clang++ is installed.");
        _e.push(Compilers::CLANG);
    }
    // if subprocess("msvc", "-v").is_ok() {
    //     println!("{}", "Microsoft vs comiler is installed.");
    //     _e.push(Compilers::CLANG);
    // }
    if subprocess("make", "-v").is_ok() {
        println!("{}", "Make is installed.");
        _e.push(Compilers::CLANG);
    }
    _e
}

