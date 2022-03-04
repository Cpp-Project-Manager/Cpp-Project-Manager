#pragma once

#include <bits/stdc++.h>
#include <unistd.h>
#include <windows.h>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <simpio/simpio.h>

#include "../headers/conv.hpp"
#include "../headers/json.hpp" // nlohmann json

// Info: So what create needs to do, is to create, ofc, c++ projects that can be opened stright from the command line anywhere. It also needs to list dependencies used in that project. Information can and will be stored in either a json file or NBT file and retrieved using either Nlohmann Json [https://github.com/nlohmann/json] or nbt-cpp [https://github.com/handtruth/nbt-cpp].
/**
 *  keypoints:
 * ? Can be opened in specified code editor from any command line
 * ! above needs to impliment a path handler
 * */

/**
 * @brief  Create Class: Relevant to Cp Create.
 * @param projectName: std::string
 * @param createFolder(): void
 * @param NewProject(): void
 */
class Create {
    int ans;
    std::string folderName, answer, folderCreate = "md ", folder, projectName;
    nlohmann::json cpconfig;

    const char* create = R"(
[0] Back to main menu.
[1] New Project
[2] Remove Project
[5] Exit
)";

public:
    Create(){};
    void CreationOptions(){
        fmt::print(fg(fmt::color::cyan), "\nWhat do you want to create?");
        fmt::print(fg(fmt::color::golden_rod), create);
        std::cin >> ans;
        switch(ans){ 
            case 0: system("cp"); break;
            case 1: NewProject(); break;
            case 2: fmt::print("NewFile()"); break;
            case 3: fmt::print("NewFileDir()"); break;
            case 5: goto br; break;
            default: fmt::print("command: {} is unknown", ans); break;
            br:
                break;
        }
    };

    void config_write(){
        std::ofstream cpc("cpconfig.json");
        cpc << std::setw(4) << cpconfig << std::endl;
    };

    void NewProject(){
        fmt::print(fg(fmt::color::cyan), "> New Project! <\n");
        fmt::print(fg(fmt::color::golden_rod), "Project name: ");
        std::cin >> projectName;
        cpconfig["project"] = projectName;
        cpconfig["lang"] = "c++/cpp";
        cpconfig["path"] = std::filesystem::current_path();
        folder = folderCreate + projectName;
        std::system(folder.c_str());

        config_write();
        if(std::filesystem::exists("cpconfig.json")){
            fmt::print(fg(fmt::color::aquamarine), "Configuration Updated!\n");
        }
        system("cp");
    };

    
};
