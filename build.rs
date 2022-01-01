fn main() {
    cc::Build::new()
    .cpp(true)
    .file("src/cp.cpp")
    .file("src/cpfc.cpp")
    .file("src/cpfd.cpp")
    .file("src/cpfile.cpp")
    .cpp_link_stdlib("libstdc++")
    .compile("cp")
}