#include <iostream>
#include <windows.h>
#include <filesystem>

#include <fmt/core.h>
#include <fmt/color.h>
#include <src/format.cc>

#ifndef FMT_CORE_H_ // Use this and comment the one above out.
#include "../fmt/include/fmt/core.h"
#include "../fmt/include/fmt/color.h"
#include "../fmt/src/format.cc"
#endif

#include "../headers/misc.hpp"
#include "../headers/argCheck.hpp"
#include "../headers/add.hpp"
#include "../headers/create.hpp"
#include "../headers/projects.hpp"
#include "../headers/cp.hpp"
using namespace conv;

namespace fs = std::filesystem;

#ifndef ENABLE_VIRTUAL_TERMINAL_PROCESSING
const ENABLE_VIRTUAL_TERMINAL_PROCESSING = 0x0004;
#endif

int main(int argc, char *argv[]){
    // Create's Class Object
    Create create;
    Projects projects;

    // Create configuration file.
    if(!fs::exists("cpconfig.json")){
        #ifdef _WIN32
            system("type NUL > cpconfig.json"); 
        #endif
       #if __APPLE__ || __linux__
            system("touch cpconfig.json")
       #endif
    }

    if(argCheck("cp", argc, argv, 1, 0)) // note: Implimented
        cp::cp();
        
    if(argCheck("create", argc, argv)) //note: 1/2 implimented
        create.CreationOptions();
    
    if(argCheck("projects", argc, argv)) //? Began implimentation
        projects.ProjectsOptions();
    
    if(argCheck("add", argc, argv)) //! Unimplimented
        fmt::print("CP Add Called");

    if(argCheck("help", argc, argv)) // info: Implimented
        fmt::print(fg(fmt::color::sky_blue), cmdInformation::help);

    if( _stricmp(argv[1], "new") == 0 && argc >= 2){
        create.New(argv); // note: Implimented
    }
        
}