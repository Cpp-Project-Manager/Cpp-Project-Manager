#include <iostream>
#include <windows.h>
#include <filesystem>

#include <fmt/core.h>
#include <fmt/color.h>
#include <src/format.cc>

//if using make, you can remove "../fmt/"
#ifndef FMT_CORE_H_ // Use this and comment the one above out.
#include "../fmt/include/fmt/core.h"
#include "../fmt/include/fmt/color.h"
#include "../fmt/src/format.cc"
#endif

//if not using make, add "../headers/"
#include "misc.hpp"
#include "argCheck.hpp"
#include "create.hpp"
using namespace conv;

#ifndef ENABLE_VIRTUAL_TERMINAL_PROCESSING
const ENABLE_VIRTUAL_TERMINAL_PROCESSING = 0x0004;
#endif

//todo: impliment cmd style.
//todo: impliment a faulty command reply
//note: to compile to dist folder, run `make -f dist.make` in root dir

int main(int argc, char *argv[]){
    // Create's Class Object
    Create create;

    if(argCheck("create", argc, argv))
        create.NewProject();

    if(argCheck("help", argc, argv))
        fmt::print(fg(fmt::color::sky_blue),"{}", cmdInformation::help);

    if( _stricmp(argv[1], "new") == 0 && argc >= 2){
        create.New(argv);
    }
}