#include <iostream>
#include <windows.h>
#include <filesystem>

#ifndef FMT_CORE_H_
#include <fmt/core.h>
#include <fmt/color.h>
#include <src/format.cc>
#endif

#ifndef FMT_CORE_H_
#include "../fmt/core.h"
#include "../fmt/color.h"
#include "../src/format.cc"
#endif

#include "../headers/misc.hpp"
#include "../headers/projectManager.hpp"
#include "../headers/conv.hpp"
#include "../headers/argCheck.hpp"
#include "../headers/add.hpp"
#include "../headers/create.hpp"
#include "../headers/projects.hpp"
#include "../headers/cp.hpp"

using namespace arguments;
using namespace conv;

namespace fs = std::filesystem;

#ifndef ENABLE_VIRTUAL_TERMINAL_PROCESSING
const ENABLE_VIRTUAL_TERMINAL_PROCESSING = 0x0004;
#endif

int main(int argc, char *argv[]){
    // Create's Class Object
    Create project;

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
        
    if(argCheck("create", argc, argv)) //note: Being implimented
        project.CreationOptions();
    
    if(argCheck("projects", argc, argv)) //! Unimplimented
        fmt::print("CP Projects Called");
    
    if(argCheck("add", argc, argv)) //! Unimplimented
        fmt::print("CP Add Called");

    if(argCheck("help", argc, argv)) // note: Implimented
        std::cout << cmdInformation::help << std::endl;
    
    
}