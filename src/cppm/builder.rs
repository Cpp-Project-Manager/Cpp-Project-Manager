#![allow(dead_code)]
use std::process::*;
pub enum Compilers {
    Gcc,
    Clang,
    Msvc,
    Gpp,
    Clangpp,
}

pub fn subprocess(process: &str, arg: &str) -> std::io::Result<ExitStatus> {
    Command::new(process)
        .arg(arg)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
}

pub fn c() -> std::io::Result<Compilers> {
    if subprocess("clang", "-v").is_ok() {
        println!("Clang is installed.");
        Ok(Compilers::Clang)
    } else if subprocess("g++", "-v").is_ok() {
        println!("G++ is installed.");
        Ok(Compilers::Gcc)
    } else {
        println!("Neither G++ or Clang++ is installed.");
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Neither Gcc or Clang is installed.",
        ))
    }
}

pub fn cpp() -> std::io::Result<Compilers> {
    if subprocess("clang++", "-v").is_ok() {
        println!("Clang++ is installed.");
        Ok(Compilers::Clangpp)
    } else if subprocess("g++", "-v").is_ok() {
        println!("G++ is installed.");
        Ok(Compilers::Gpp)
    } else {
        println!("Neither G++ or Clang++ is installed.");
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Neither G++ or Clang++ is installed.",
        ))
    }
}
