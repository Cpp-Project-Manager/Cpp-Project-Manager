#include <iostream>
#include <windows.h>

#include "../headers/misc.hpp"
#include "../headers/projectManager.hpp"
#include "../headers/conv.hpp"
#include "../headers/argCheck.hpp"

#include "../headers/add.hpp"
#include "../headers/create.hpp"
#include "../headers/projects.hpp"
using namespace arguments;


int main(int argc, char *argv[]){
    Create project;

    if(argCheck("cp", argc, argv, 1, 0)){ // note: Implimented
        std::cout << "Welcome to Creation Project. What would you like to do?: " << std::endl;
        std::cout << cmdInformation::help << std::endl;
    }
    if(argCheck("create", argc, argv)){ //! Unimplimented
        project.NewProject();
    }
    if(argCheck("projects", argc, argv)){ //! Unimplimented
        std::cout << "CP Projects Called" << std::endl;
    }
    if(argCheck("add", argc, argv)){ //! Unimplimented
        std::cout << "CP Add Called" << std::endl;
    }
    if(argCheck("help", argc, argv)){ // note: Implimented
        std::cout << cmdInformation::help << std::endl;
    }

    
}