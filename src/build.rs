use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, read_to_string},
    path::Path,
    process::*,
    str,
    time::Instant,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalConfig {
    pub project: HashMap<String, String>,
}

const FLAGS: &str = "-Wall -Wpedantic -Werror -Wshadow -Wformat=2 -Wconversion -Wunused-parameter";

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

// note: dynamically "include"
// note: add `flags_all = bool`, `flags = ""`
// note: look for a logger lib
// note: optimize for smart object building headerfiles in the future
// note: add build --release, -O3
// note: add building messages
// note: add git_init integration
// warning: dont forget linux support
pub fn build() {
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
    let start = Instant::now();
    if !Path::new("Cppm.toml").exists() {
        println!("Cppm project isnt in current directory!");
        exit(0);
    }
    let cppm: Build = toml::from_str(&read_to_string("Cppm.toml").unwrap()).unwrap();
    let includes: Vec<&str> = cppm.project["include"].split(",").collect();
    let src = cppm.project["src"].clone();

    let build = format!(
        "g++ {} {src} -o build/{} {FLAGS}",
        include(includes.clone()),
        cppm.project["name"].clone()
    );
    // println!("{}", build.clone());

    fs::create_dir_all("build").ok();
    use std::io::{self, Write};
    let out = Command::new("powershell").arg(build).output().unwrap();
    if !out.status.success() {
        io::stdout().write_all(&out.stdout).unwrap();
        io::stderr().write_all(&out.stderr).unwrap();
        exit(0);
    }
    println!(
        "    {} dev [unoptimized] in {:?}",
        "Finished".bright_green(),
        start.elapsed()
    );
}

// warning: get output and print to console
pub fn run() {
    let cppm: Build = toml::from_str(&read_to_string("Cppm.toml").unwrap()).unwrap();
    build();
    let l: LocalConfig = toml::from_str(&std::fs::read_to_string("Cppm.toml").unwrap()).unwrap();
    let name = format!("build/{}.exe", l.project["name"]);
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
    let run = format!("build/{}.exe", cppm.project["name"]);
    let out = Command::new("powershell")
        .arg(run.clone())
        .output()
        .unwrap();
    let out = str::from_utf8(&out.stdout).unwrap();
    println!("{}", out.trim());
}
