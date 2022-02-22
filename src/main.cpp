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
using namespace conv;


int main(int argc, char *argv[]){
    Create project;

    if(argCheck("cp", argc, argv, 1, 0)){ // note: Implimented
        print("Welcome to Creation Project. What would you like to do?: ");
        std::cout << cmdInformation::menu << std::endl;
        for(;;){
            std::cin >> cmdInformation::menuAnswer;
            switch(cmdInformation::menuAnswer){
                case 1: print("Cp Create"); break;
                case 2: print("Cp Projects"); break;
                case 3: print("Cp Add"); break;
                case 4: 
                print("Cp Help: ");
                print(cmdInformation::help); break;
                case 5: return 0; break;
            }
            print("What else would you like to do? (5 to exit)");
        }
    }
    if(argCheck("create", argc, argv)){ //! Unimplimented
        project.NewProject();
    }
    if(argCheck("projects", argc, argv)){ //! Unimplimented
        print("CP Projects Called");
    }
    if(argCheck("add", argc, argv)){ //! Unimplimented
        print("CP Add Called");
    }
    if(argCheck("help", argc, argv)){ // note: Implimented
        std::cout << cmdInformation::help << std::endl;
    }

    
}