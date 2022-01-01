fn main() {
    cc::Build::new()
        .cpp(true)
        .cpp_link_stdlib("libstdc++")
        .file("cp.cpp")
        .compile("cp")
        .file("cpfc.cpp")
        .compile("cpfc")
        .file("cpfd.cpp")
        .compile("cpfd")
        .file("cpfile.cpp"
        .compile("cpfile"));
}