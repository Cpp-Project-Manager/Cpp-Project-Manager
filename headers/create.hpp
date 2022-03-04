#pragma once

#include <windows.h>
#include <filesystem>
#include <fstream>
#include <iostream>

#include "json.hpp"

// Info: So what create needs to do, is to create, ofc, c++ projects that can be opened stright from the command line anywhere. It also needs to list dependencies used in that project. Information can and will be stored in either a json file or NBT file and retrieved using either Nlohmann Json [https://github.com/nlohmann/json] or nbt-cpp [https://github.com/handtruth/nbt-cpp].
/**
 *  keypoints:
 * ? Can be opened in specified code editor from any command line
 * ! above needs to impliment a path handler
 * */

/**
 * @brief  Create Class: Relevant to Cp Create.
 * @param projectName: string
 * @param createFolder(): void
 * @param NewProject(): void
 */
class Create {
private:
    int ans, projectTemplateAns;

    std::string folderName, answer, folderCreate = "md ", folderRemove = "rmdir", folder, projectName, editor;

    nlohmann::json cpconfig;

    const char* create = R"(
[0] Main Menu.
[1] New Project
[2] Remove Project
[5] Exit
)";

public:
    Create() = default;
    void CreationOptions(){
        fmt::print(fmt::emphasis::underline | fg(fmt::color::cyan), "\nWhat do you want to create?\n");
        fmt::print(fg(fmt::color::golden_rod), create);
        fmt::print(fg(fmt::color::sky_blue), "\nYour Option: ");
        std::cin >> ans;
        switch(ans){ 
            case 0: system("cp"); break;
            case 1: NewProject(); break;
            case 2: RemoveProject(); break;
            case 3: fmt::print("NewFileDir()"); break;
            case 5: break;
            default: fmt::print("command: {} is unknown", ans); break;
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
        cpconfig["path"] = std::filesystem::current_path();
        cpconfig["lang"] = "cpp";
        cpconfig["project"] = projectName;
        folder = folderCreate + projectName;
        system(folder.c_str());

        std::string path = cpconfig["path"].get<std::string>();

        fmt::print(fg(fmt::color::golden_rod), "Editor: ");
        std::cin >> editor;
        cpconfig["editor"] = editor;

        fmt::print(fg(fmt::color::cyan), "\nProject Template: ");
        fmt::print(fg(fmt::color::golden_rod), cmdInformation::projectTemplates);
        std::cin >> projectTemplateAns;
        switch(projectTemplateAns){
            case 1: goto cont; break;
            case 2: SrcTemp(path); break;
            case 3: ClassTemp(path); break;
            case 4: break;
            case 5: break;
        }
        cont:
            config_write();
            if(std::filesystem::exists("cpconfig.json")){
                fmt::print(fg(fmt::color::aquamarine), "Configuration Updated!\n");
            }
            system("cp");
        
    };

    void RemoveProject(){};
    void SrcTemp(std::string path){
        std::string projectPath = path + "\\" + projectName;
        std::string mainCpp = projectPath + "\\src\\main.cpp";
        std::string command = "cd " + projectPath + " && mkdir src && mkdir include && cd src && type NUL > main.cpp && cd ../ && cd include && type NUL > main.hpp";
        system(command.c_str());
        std::ofstream main_cpp(mainCpp);
        main_cpp << conv::cppboiler;
        
    }
    void ClassTemp(std::string path){
        std::string projectPath = path + "\\" + projectName;
        std::string mainHpp = projectPath + "\\include\\main.hpp";
        std::ofstream main_hpp(mainHpp);
    }
    
};

// void removeFolder(){
//   std::string check, name, concat, cmd = "del ", scmd, nconcat;
//   std::cout << "----------------CP Remover----------------" << std::endl;
//   std::cout << "What do you want to delete? Options: File or Folder."
//             << std::endl;
//   std::cin >> check;
//   if (check == "File" || check == "file") {
//     std::cout << "What file do you want to delete?" << std::endl;
//     std::cin >> name;
//     concat = cmd + name;
//     system(concat.c_str());
//   } else if (check == "Folder" || check == "folder") {
//     std::cout << "What folder do you want to delete?" << std::endl;
//     std::cin >> name;
//     scmd = "rmdir /s ";
//     nconcat = scmd + name;
//     system(nconcat.c_str());
//   }
// }