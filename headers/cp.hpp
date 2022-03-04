#pragma once

#include <iostream>

#include "create.hpp"
#include "misc.hpp"

Create project;

namespace cp {
    void cp(){
        fmt::print(fmt::emphasis::underline | fg(fmt::color::cyan), "\nWelcome to Creation Project. What would you like to do?: ");
        fmt::print(fg(fmt::color::golden_rod), cmdInformation::menu);
        for(;;){
            std::cin >> cmdInformation::menuAnswer;
            switch(cmdInformation::menuAnswer){
                case 1: project.CreationOptions(); break;
                case 2: fmt::print("Cp Projects"); break;
                case 3: fmt::print("Cp Add"); break;
                case 4:
                fmt::print(fg(fmt::color::cyan), "All cp arguments can be called manually or called by using `cp`'s menu.");
                fmt::print(fg(fmt::color::golden_rod),cmdInformation::help); break;
                case 5: goto br; break;
                default: fmt::print("command: {} not found.", cmdInformation::menuAnswer);
            }
            br: break;
        }
    }
}