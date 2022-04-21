#include <iostream>
#include <windows.h>
#include <filesystem>

#include <fmt/core.h>
#include <fmt/color.h>
#include <src/format.cc>
#include <parse_args.h>

#ifndef FMT_CORE_H_ // Use this and comment the one above out.
#include "../fmt/include/fmt/core.h"
#include "../fmt/include/fmt/color.h"
#include "../fmt/src/format.cc"
#endif

#include "include/misc.hpp"
#include "include/create.hpp"
using namespace conv;

//todo: impliment a faulty command reply

int main(int argc, char *argv[]){
    ParseArgs args = ParseArgs(argc, argv);
    Create create;

    if(args.DefaultParse("create"))
        create.NewProject();

    if(args.DefaultParse("help"))
        fmt::print(fg(fmt::color::sky_blue),"{}", cmdInformation::help);

    if(args.Parse("new", 1)){
        create.New(argv);
    }
    /*todo:
    Open anywhere from terminal using toml++ and (if need be, create)dirs.
    Impliment a Run, Build, Clean and Release using makefile.
    Remove cppm create.
    Crossplatform code.
    */
}