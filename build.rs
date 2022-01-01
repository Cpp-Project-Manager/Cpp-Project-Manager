fn cp(){
    cc::Build::new()
        .cpp(true)
        .file("cp.cpp")
        .cpp_link_stdlib("libstdc++")
        .compile("cp")
}

fn cpfc(){
    cc::Build::new()
        .cpp(true)
        .file("cpfc.cpp")
        .cpp_link_stdlib("libstdc++")
        .compile("cpfc")
}

fn cpfd(){
    cc::Build::new()
        .cpp(true)
        .file("cpfd.cpp")
        .cpp_link_stdlib("libstdc++")
        .compile("cpfd")
}

fn cpfile(){
    cc::Build::new()
        .cpp(true)
        .file("cpfile.cpp")
        .cpp_link_stdlib("libstdc++")
        .compile("cpfile")
}

fn main() {
    cp();
    cpfc();
    cpfd();
    cpfile();
}