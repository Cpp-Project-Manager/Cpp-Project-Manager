#![allow(dead_code)]
use std::process::*;
pub enum Compilers {
    Gcc,
    Clang,
    Msvc,
    Gpp,
    Clangpp
}

fn subprocess(process: &str, arg: &str)-> std::io::Result<ExitStatus> {
    Command::new(process).arg(arg).stdout(Stdio::null()).stderr(Stdio::null()).status()
}

pub fn compiler_check()-> Vec<Compilers>{
    let mut _e: Vec<Compilers> = Vec::new();
    if subprocess("gcc", "-v").is_ok() {
        println!("GCC is installed.");
        _e.push(Compilers::Gcc);
    }
    if subprocess("g++", "-v").is_ok() {
        println!("G++ is installed.");
        _e.push(Compilers::Gpp);
    }
    if subprocess("clang", "-v").is_ok() {
        println!("Clang is installed.");
        _e.push(Compilers::Clang);
    }
    if subprocess("clangpp", "-v").is_ok() {
        println!("Clang++ is installed.");
        _e.push(Compilers::Clang);
    }
    _e
}


