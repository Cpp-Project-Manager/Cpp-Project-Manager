#![allow(unused_assignments)]
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, read_to_string},
    io::{self, Write},
    path::Path,
    process::*,
    str,
    time::Instant,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalConfig {
    pub project: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Build {
    project: HashMap<String, String>,
}

pub fn include(folders: Vec<&str>) -> String {
    let mut return_string: Vec<&str> = Vec::new();
    for i in folders {
        return_string.push(i);
    }
    format!("-I{}", return_string.join(" -I"))
}

#[allow(dead_code)]
pub fn sources() {}

#[allow(dead_code)]
pub fn compiler() {}
#[allow(dead_code)]
pub fn lib() {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Def {
    compilers: HashMap<String, String>,
}

pub fn defaults_file() -> String {
    let defaultsdir = dirs::config_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        .replace('"', "")
        .replace('\\', "/");
    format!("{}/cppm/defaults.toml", defaultsdir)
}

// note: add `flags_all = bool`, `flags = ""`
// note: optimize for smart object building headerfiles in the future
// note: add git_init integration
pub fn build(release: bool) {
    let start = Instant::now();
    if !Path::new("Cppm.toml").exists() {
        println!("Cppm project isnt in current directory!");
        exit(0);
    }
    let mut target = String::new();
    let mut build_t = String::new();

    let l: LocalConfig = toml::from_str(&std::fs::read_to_string("Cppm.toml").unwrap()).unwrap();
    println!(
        "   {} {} v{} ({:?})",
        "Compiling".bright_green(),
        l.project["name"],
        l.project["version"],
        std::fs::canonicalize(".")
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .replace('\\', "/")
            .trim()[4..]
            .to_owned()
    );

    let cppm: Build = toml::from_str(&read_to_string("Cppm.toml").unwrap()).unwrap();
    let compiler: Def = toml::from_str(&read_to_string(&defaults_file()).unwrap()).unwrap();

    let includes: Vec<&str> = cppm.project["include"].split(",").collect();
    let src = cppm.project["src"].clone();

    fs::create_dir_all("build").ok();

    let out: Output;
    if release == true {
        out = Command::new(&compiler.compilers["cpp"])
            .args([
                "-o".to_string(),
                format!("build/{}", cppm.project["name"].clone()),
                src.clone(),
                include(includes.clone()),
                "-Wall".to_string(),
                "-Wpedantic".to_string(),
                "-Werror".to_string(),
                "-Wshadow".to_string(),
                "-Wformat=2".to_string(),
                "-Wconversion".to_string(),
                "-Wunused-parameter".to_string(),
                "-O3".to_string(),
            ])
            .output()
            .unwrap();
        target = "optimized".to_string();
        build_t = "release".to_string();
    } else {
        out = Command::new(&compiler.compilers["cpp"])
            .args([
                "-o".to_owned(),
                format!("build/{}", cppm.project["name"].clone()),
                src.clone(),
                include(includes.clone()),
                "-Wall".to_string(),
                "-Wpedantic".to_string(),
                "-Werror".to_string(),
                "-Wshadow".to_string(),
                "-Wformat=2".to_string(),
                "-Wconversion".to_string(),
                "-Wunused-parameter".to_string(),
            ])
            .output()
            .unwrap();
        target = "unoptimized".to_owned();
        build_t = "dev".to_string();
    }
    if !out.status.success() {
        io::stdout().write_all(&out.stdout).unwrap();
        io::stderr().write_all(&out.stderr).unwrap();
        exit(0);
    }
    println!(
        "    {} {build_t} [{target}] target(s) in {:?}",
        "Finished".bright_green(),
        start.elapsed()
    );
}

pub fn run(release: bool) {
    if !Path::new("Cppm.toml").exists() {
        println!("Cppm project isnt in current directory!");
        exit(0);
    }
    let cppm: Build = toml::from_str(&read_to_string("Cppm.toml").unwrap()).unwrap();
    build(release);
    let l: LocalConfig = toml::from_str(&std::fs::read_to_string("Cppm.toml").unwrap()).unwrap();

    #[cfg(windows)]
    let name = format!("build/{}.exe", l.project["name"]);
    #[cfg(unix)]
    let name = format!("build/{}", l.project["name"]);

    println!(
        "     {} `{}`",
        "Running".bright_green(),
        std::fs::canonicalize(&name)
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .replace('\\', "/")
            .trim()[4..]
            .to_owned()
    );

    let run = format!("build/{}", cppm.project["name"]);
    let out = Command::new(run).output().unwrap();
    io::stdout().write_all(&out.stdout).unwrap();
    io::stderr().write_all(&out.stderr).unwrap();
}
