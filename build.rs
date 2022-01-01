fn main() {
    cc::Build::new()
        .cpp(true)
        .cpp_link_stdlib("libstdc++")
        .file("cp.cpp")
        .file("bar.c")
        .compile("foo");
}