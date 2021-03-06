<h1 align="center"> C++ Project Manager </h1>

<h4 align="center"> The C++ equivalent to cargo. </h4>

<p align="center">
  <a href="https://github.com/Cpp-Project-Manager/Cpp-Project-Manager/actions">
    <img src="https://img.shields.io/github/workflow/status/Cpp-Project-Manager/Cpp-Project-Manager/Rust/v2.2.1?style=for-the-badge"
         alt="WorkFlow">
  </a>
  <a href="https://github.com/Cpp-Project-Manager/Cpp-Project-Manager/releases/tag/v3.0.3"><img src="https://img.shields.io/github/v/release/Cpp-Project-Manager/Cpp-Project-Manager?style=for-the-badge"></a>
  <a href="https://crates.io/crates/cppm">
      <img src="https://img.shields.io/crates/d/cppm?label=Crate%20Downloads&logo=Rust&style=for-the-badge">
  </a>
  <a href="https://github.com/Cpp-Project-Manager/Cpp-Project-Manager/releases/tag/v2.2.1">
    <img src="https://img.shields.io/github/downloads/Cpp-Project-Manager/Cpp-Project-Manager/total?label=Github%20Downloads&logo=Github&style=for-the-badge">
  </a>
  <a href="https://crates.io/crates/cppm">
  </a>
</p>

<p align="center">
  <a href="#key-features">Key Features</a> •
  <a href="#getting-started">Getting Started</a> •
  <a href="https://github.com/Cpp-Project-Manager/Cpp-Project-Manager/wiki/Usage">How To Use</a> •
  <a href="#download">Download</a> •
  <a href="#license">License</a>
</p>

## Key Features
* Create a C++ project with a single command.
* Manage Project Locations.
* Opening Projects from any command line
* Minimal C support 

### Download. 
There's three ways to get cppm:
1. [Download](https://github.com/Cpp-Project-Manager/Cpp-Project-Manager/releases/latest) the build for windows.
2. Install with Cargo `cargo install cppm`
3. Get with our [custom installer](https://github.com/maou-shimazu/cppm-installer)

### Getting Started
The first thing needed when cppm has just been installed is to run `cppm --config`, and it'll ask for your default editor.
You should see something like this:
```
$ cppm --config
Default editor: nvim
Clang is installed.
Clang++ is installed.
Location: /some/path/.cppm/defaults.toml
```
After that you are free to start using cppm.

#### For usage instructions run `cppm --help` or refer to the [wiki](https://github.com/Cpp-Project-Manager/Cpp-Project-Manager/wiki).

## License
[MIT](https://github.com/Cpp-Project-Manager/Cpp-Project-Manager/blob/main/LICENSE)

---
