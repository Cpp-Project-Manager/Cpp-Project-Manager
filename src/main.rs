mod build;
mod cppm;
use clap::{Parser, Subcommand};
use colored::Colorize;
use cppm::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Lists all projects configured with cppm
    #[clap(short, long)]
    list: bool,

    /// Generate C files instead of C++ files
    #[clap(short)]
    c: bool, //warning: move c to build, new, init

    /// Configure cppm defaults
    #[clap(short = 'g', long)]
    config: bool,

    /// Initialize cppm in the current directory
    #[clap(short, long)]
    init: bool,

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
    },
    // Initialize a cppm project in current directory
    Init {

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
}

fn main() {
    let args = Args::parse();
    #[cfg(windows)]
    let _enabled = ansi_term::enable_ansi_support();

    if args.list {
        misc::list_projects()
    }
    if args.config {
        cppm::defaults()
    }
    if args.clean {
        Cppm::clean()
    }
    if args.init {
        if args.c {
            Cppm::initialize("c").ok();
        } else {
            Cppm::initialize("cpp").ok();
        }
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
        Some(Command::New { name, editor }) => {
            if !args.git {
                cppm::git_init();
            }
            if args.c {
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
        }
        // warning: check
        Some(Command::Init {}) => {
            if !args.git {
                cppm::git_init();
            }
            if args.c {
                Cppm::initialize("c").ok();
            } else {
                Cppm::initialize("cpp").ok();
            }
        }
        Some(Command::Build { release }) => {
            build::build(release);
        }
        Some(Command::Clean) => {
            Cppm::clean();
        }
        // note: pass extra args through run
        Some(Command::Run { release }) => {
            build::run(release);
        }
        None => (),
    }
}
