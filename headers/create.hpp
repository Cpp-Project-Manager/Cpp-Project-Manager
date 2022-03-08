#pragma once

#include <windows.h>
#include <filesystem>
#include <fstream>
#include <iostream>

#include "json.hpp"

/**
 *  keypoints:
 * ? Can be opened in specified code editor from any command line
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

    std::string folderName, answer, folderCreate = "md ", folderRemove = "rmdir", folder, projectName, editor, className;

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
            default: fmt::print("command: {} is unknown", ans); CreationOptions(); break;
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
        std::string path = std::filesystem::current_path().string() + "\\" + projectName;
        folder = folderCreate + projectName;
        system(folder.c_str());

        fmt::print(fg(fmt::color::cyan), "\nProject Template: ");
        fmt::print(fg(fmt::color::golden_rod), cmdInformation::projectTemplates);
        std::cin >> projectTemplateAns;
        switch(projectTemplateAns){
            case 1: goto cont; break;
            case 2: SrcTemp(path); break;
            case 3: ClassTemp(path); break;
            case 4: break;
            case 5: fmt::print(fg(fmt::color::medium_violet_red), "Configuration Failed!\n"); exit(0); break;
        }
        cont:
            cpconfig = {
                projectName, path
            };
            
            config_write();
            fmt::print(fg(fmt::color::aquamarine), "Configuration Updated!\n");
            system("cp");
        
    };
    void write_to_cpp(std::string path){
        std::string cpp = path + "\\src\\main.cpp";
        std::ofstream main_cpp(cpp);
        main_cpp << conv::cppboiler;
    }
    void write_to_hpp(std::string path){
        std::string hpp = path + "\\include\\"+ className + ".hpp";
        std::ofstream main_hpp(hpp);
        main_hpp << conv::headerBoiler;
    }
    // templates
    void SrcTemp(std::string path){
        std::string command = "cd " + path + " && mkdir src && mkdir include && cd src && type NUL > main.cpp"; // && cd ../ && cd include && type NUL > main.hpp";
        system(command.c_str());
        write_to_cpp(path);
    }
    
    void ClassTemp(std::string path){
        fmt::print("Name of main class: ");
        std::cin >> className;
        std::system(fmt::format("cd {0} && mkdir src && mkdir include && cd src && type NUL > main.cpp && type NUL > {1}.cpp && cd ../ && cd include && type NUL > {1}.hpp", path, className).c_str());
        write_to_hpp(path);
        write_to_cpp(path);
    }

    void RemoveProject(){};

    void New(char *argv[]){
        char *word = argv[2];
        char *editor = argv[3];
        system(fmt::format("mkdir {}", word).c_str());
        SrcTemp(std::filesystem::current_path().string() + "\\" + word);

        if (editor != NULL)
            Open(fmt::format("{}\\{}", std::filesystem::current_path().string(), word), editor);

        fmt::print(fg(fmt::color::sky_blue), "Created binary application `{}`.", word);

    };

    void Open(std::string path, const char *editor){
        system(fmt::format("cd {} && {} .", path, editor).c_str());
    };

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