mod cppm;
use cppm::*;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Lists all projects configured with cppm
    #[clap(short, long)]
    list: bool,

    /// Generate C files instead of C++ files
    #[clap(short, long)]
    c: bool,

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
    ini: bool,

    /// Remove a cppm project
    #[clap(short, long)]
    remove: Option<String>,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Clone)]
enum Command {
    /// Open a cppm project
    Open { name: String, editor: Option<String> },
    /// Creates a new cppm project
    New { name: String, editor: Option<String> }
}

fn main() {
    let args = Args::parse();
    #[cfg(windows)]
    let _enabled = ansi_term::enable_ansi_support();
    
    if args.list { misc::list_projects() }
    if args.config { cppm::defaults() }
    if args.clean { Cppm::clean() }
    if args.init {
        if args.c {
            Cppm::initialize("c").ok();
        } else {
            Cppm::initialize("cpp").ok();
        }
    }
    if args.ini {
        std::process::Command::new("notepad")
            .arg(misc::configfile())
            .spawn()
            .expect("Couldn't start notepad.");
        #[cfg(unix)]
        std::process::Command::new("nvim")
            .arg(misc::configfile())
            .spawn()
            .expect("Couldnt start Nvim.");
        println!("location: {}", misc::configfile())
    }

    if let Some(remove) = args.remove {
        cppm::remove(remove);
    }

    match args.command {
        Some(Command::Open { name, editor })  => {
            if editor.is_none() {
                // note: do config stuff, get default ed if exists
            } else {
              Cppm::open(name, editor.unwrap());  
            }
        }
        Some(Command::New { name, editor }) => {
            if args.c {
                Cppm::spawn(name.clone(), editor.unwrap_or("null".to_string()), "c");
                println!(
                    "    {} C project `{}`",
                    "Created".bright_green(),
                    name
                );
            } else {
                Cppm::spawn(name.clone(), editor.unwrap_or("null".to_string()), "cpp");
                println!(
                    "    {} C++ project `{}`",
                    "Created".bright_green(),
                    name
                );
            }
        }
        None => (),
    }
    
}
