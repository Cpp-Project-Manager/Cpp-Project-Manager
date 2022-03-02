#include <iostream>
#include <windows.h>
#include <fmt/core.h>
#include <fmt/color.h>
#include <src/format.cc>
#include <filesystem>

#include "../headers/misc.hpp"
#include "../headers/projectManager.hpp"
#include "../headers/conv.hpp"
#include "../headers/argCheck.hpp"
#include "../headers/add.hpp"
#include "../headers/create.hpp"
#include "../headers/projects.hpp"

using namespace arguments;
using namespace conv;

namespace fs = std::filesystem;

int main(int argc, char *argv[]){
    // Create's Class Object
    Create* project;

    // Create configuration file.
    if(!fs::exists("cpconfig.json")){
        #ifdef _WIN32
            system("md cpconfig.json"); 
        #endif
       #if __APPLE__ || __linux__
            system("touch cpconfig.json")
       #endif
    }

    if(argCheck("cp", argc, argv, 1, 0)){ // note: Implimented
        fmt::print(fg(fmt::terminal_color::cyan), "Welcome to Creation Project. What would you like to do?: ");
        fmt::print(fg(fmt::color::dark_golden_rod), cmdInformation::menu);
        for(;;){
            std::cin >> cmdInformation::menuAnswer;
            switch(cmdInformation::menuAnswer){
                case 1: project.CreationOptions(); break;
                case 2: fmt::print("Cp Projects"); break;
                case 3: fmt::print("Cp Add"); break;
                case 4: 
                fmt::print("Cp Help: ");
                fmt::print(cmdInformation::help); break;
                case 5: return 0; break;
            }
            fmt::print("What else would you like to do? (5 to exit)");
        }
    }
    if(argCheck("create", argc, argv)){ //! Unimplimented
<<<<<<< HEAD
        project.CreationOptions();
=======
        project->NewProject();
>>>>>>> 922e56b77430ab269bf424895b0571c68c31b737
    }
    if(argCheck("projects", argc, argv)){ //! Unimplimented
        fmt::print("CP Projects Called");
    }
    if(argCheck("add", argc, argv)){ //! Unimplimented
        fmt::print("CP Add Called");
    }
    if(argCheck("help", argc, argv)){ // note: Implimented
        std::cout << cmdInformation::help << std::endl;
    }
    
}