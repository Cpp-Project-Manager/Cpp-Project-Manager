<h1 align="center"> C++ Project Manager </h1>

<h4 align="center"> The C++ alternative to cargo. </h4>

<p align="center">
  <a href="https://github.com/Maou-Shimazu/Cpp-Project-Manager/actions">
    <img src="https://img.shields.io/github/workflow/status/Maou-Shimazu/Cpp-Project-Manager/Rust/v2.2.1?style=for-the-badge"
         alt="WorkFlow">
  </a>
  <a href="https://github.com/Maou-Shimazu/Cpp-Project-Manager/releases/tag/v2.2.1"><img src="https://img.shields.io/github/v/release/Maou-Shimazu/Cpp-Project-Manager?style=for-the-badge"></a>
  <a href="https://crates.io/crates/cppm">
      <img src="https://img.shields.io/crates/d/cppm?label=Crate%20Downloads&logo=Rust&style=for-the-badge">
  </a>
  <a href="https://github.com/Maou-Shimazu/Cpp-Project-Manager/releases/tag/v2.2.1">
    <img src="https://img.shields.io/github/downloads/Maou-Shimazu/Cpp-Project-Manager/total?label=Github%20Downloads&logo=Github&style=for-the-badge">
  </a>
  <a href="https://crates.io/crates/cppm">
  </a>
</p>

<p align="center">
  <a href="#key-features">Key Features</a> •
  <a href="#how-to-use">How To Use</a> •
  <a href="#download">Download</a> •
  <a href="#license">License</a>
</p>

## Key Features
* Create a C++ project with a single command.
* Manage Project Locations.
* Opening Projects from any command line

## How To Use
Note: * means a required entity, and + means an optional entity.
```powershell
# Download cppm and have it in your PATH so its available on all terminals
cppm --help/-h # Will display the help menu
cppm --version/-v # Will display the current version of cppm

# Currently available commands

# Creates a new project in the current directory and opens it in the
# specified editor if specified. No default editor available. 
cppm new <*project_name> <+editor> 

# Opens the project the specified editor if specified. 
# No default editor available.
# Supports editors that have the `{editor} .` functionality
# in the terminal. eg: Vscode, Atom, Sublime, etc.
cppm open <*project_name> <*editor> 

# Removes all files and directories inside of a project
cppm clean <*project_name>

```


## Download. 
You can [download](https://github.com/Maou-Shimazu/Cpp-Project-Manager/releases/tag/v2.2.1) the build for windows.
Or Download with cargo.
```cargo install cppm```

## License
[MIT](https://github.com/Maou-Shimazu/Cpp-Project-Manager/blob/main/LICENSE)

---
