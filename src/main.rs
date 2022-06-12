mod build;
mod cppm;
use clap::{Parser, Subcommand};
use colored::Colorize;
use cppm::*;
use human_panic::setup_panic;
use std::env;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Lists all projects configured with cppm
    #[clap(short, long)]
    list: bool,

    //note: add makefile flag, for cppm use only generating files
    /// Configure cppm defaults
    #[clap(long)]
    config: bool,

    /// Compare your current cppm version to the most recent version
    #[clap(short = 's', long)]
    status: bool,

    /// Clean up build
    #[clap(long)]
    clean: bool,

    /// View and Edit cppm config file
    #[clap(long)]
    toml: bool,

    #[clap(long = "no-git")]
    git: bool,

    /// Remove a cppm project
    #[clap(short, long)]
    remove: Option<String>,

    #[clap(short, long)]
    quiet: bool,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Clone)]
enum Command {
    /// Open a cppm project
    Open {
        name: String,
        editor: Option<String>,
    },
    /// Creates a new cppm project
    New {
        name: String,
        editor: Option<String>,
        /// Generate C files instead of C++ files
        #[clap(short)]
        c: bool,
    },
    // Initialize a cppm project in current directory
    Init {
        /// Generate C files instead of C++ files
        #[clap(short)]
        c: bool,
    },
    /// Build your project to build directory.
    Build {
        #[clap(long)]
        release: bool,
    },
    /// Clean current build
    Clean,
    /// Run project's main file
    Run {
        #[clap(long)]
        release: bool,
    },
    /// Constantly watch a file for changes and build/run when changes are detected
    Watch { filename: Option<String> },
}

fn main() {
    setup_panic!();
    let args = Args::parse();
    #[cfg(windows)]
    let _enabled = ansi_term::enable_ansi_support();

    if args.list {
        misc::list_projects()
    }
    if args.config {
        cppm::defaults()
    }
    if args.status {
        Cppm::status();
    }
    if args.clean {
        Cppm::clean()
    }
    if args.toml {
        cppm::toml();
    }
    if let Some(remove) = args.remove {
        cppm::remove(remove);
    }

    match args.command {
        Some(Command::Open { name, editor }) => {
            if editor.is_some() {
                Cppm::open(name, editor.unwrap());
            } else {
                // note: do config stuff, get default ed if exists
            }
        }
        Some(Command::New { name, editor, c }) => {
            if c {
                Cppm::spawn(
                    name.clone(),
                    editor.unwrap_or_else(|| "null".to_string()),
                    "c",
                );
                println!("    {} C project `{}`", "Created".bright_green(), name);
            } else {
                Cppm::spawn(
                    name.clone(),
                    editor.unwrap_or_else(|| "null".to_string()),
                    "cpp",
                );
                println!("    {} C++ project `{}`", "Created".bright_green(), name);
            }
            if !args.git && cppm::git_exists() {
                env::set_current_dir(name.clone()).unwrap();
                cppm::git_init();
                //env::set_current_dir("../").unwrap();
            }
        }
        // warning: check
        Some(Command::Init { c }) => {
            if c {
                Cppm::initialize("c").ok();
            } else {
                Cppm::initialize("cpp").ok();
            }
            if !args.git && cppm::git_exists() {
                cppm::git_init();
            }
        }
        Some(Command::Build { release }) => {
            build::build(release, args.quiet);
        }
        Some(Command::Clean) => {
            Cppm::clean();
        }
        Some(Command::Watch { filename }) => {
            Cppm::watch(filename.unwrap_or("src/main.cpp".to_string()));
        }
        // note: pass extra args through run
        Some(Command::Run { release }) => {
            build::run(release, args.quiet);
        }
        None => (),
    }
}
