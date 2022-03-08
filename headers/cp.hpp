#pragma once

#include <iostream>

#include "create.hpp"
#include "misc.hpp"

Create create;

namespace cp {
    void cp(){
        fmt::print(fmt::emphasis::underline | fg(fmt::color::cyan), "\nWelcome to Creation Project.\n");
        fmt::print(fg(fmt::color::golden_rod), cmdInformation::menu);
        for(;;){
            fmt::print(fg(fmt::color::sky_blue), "\nYour Option: ");
            std::cin >> cmdInformation::menuAnswer;
            switch(cmdInformation::menuAnswer){
                case 1: create.CreationOptions(); break;
                case 2: break;
                case 3: fmt::print("Cp Add"); break;
                case 4:

                fmt::print(fg(fmt::color::cyan), "\nAll cp arguments can be called manually or called by using `cp`'s menu.");
                fmt::print(fg(fmt::color::golden_rod),cmdInformation::help);
                system("cp");
                break;

                case 5: goto br; break;
                default: fmt::print("command: {} not found.", cmdInformation::menuAnswer);
            }
            br: break;
        }
    }
}