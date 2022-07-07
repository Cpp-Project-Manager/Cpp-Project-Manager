#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use fsio::file;
use std::{
    path::Path,
    collections::HashMap,
};
use std::process::{ 
    Command,
    Stdio,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalConfig {
    pub project: HashMap<String, String>,
}

pub fn create(){
    const CLANGD: &str = ".clangd";
    const CLANG_FORMAT: &str = ".clang-format";

    if !Path::new(CLANGD).exists() {
        file::ensure_exists(CLANGD).expect("could not create .cppmd.toml");
    }
    if !Path::new(CLANG_FORMAT).exists() {
        file::ensure_exists(CLANG_FORMAT).expect("could not create .cppmd.toml");
    }

    file::write_file(CLANGD, crate::templates::CLANGD.as_bytes()).expect("Could not write to .clangd.");
    file::write_file(CLANG_FORMAT, crate::templates::CLANG_FORMAT.as_bytes()).expect("Could not write to .clang-format.");
}

#[allow(dead_code)]
pub fn create_cppmd(name: Option<String>){
    let name = name.unwrap_or("".to_string());
    if !Path::new(&format!("{name}/.cppmd.toml")).exists() {
        file::ensure_exists(".cppmd.toml").expect("could not create .cppmd.toml");
    }
}