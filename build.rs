fn main() {
    cc::Build::new()
    .cpp(true)
    .file("cp.cpp")
    .file("cpfc.cpp")
    .file("cpfd.cpp")
    .file("cpfile.cpp")
    .cpp_link_stdlib("libstdc++")
    .compile("cp")
}