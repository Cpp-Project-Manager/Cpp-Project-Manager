#pragma once

#include <windows.h>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <vector>

/**
 * ? get exe path, write to $path/projects each file name.
 */

/**
 * @brief  Create Class: Relevant to Cp Create.
 * @param projectName: string
 * @param createFolder(): void
 * @param NewProject(): void
 */
class Create {
private:
    int ans, projectTemplateAns;

    std::string folderName, answer, folderCreate = "md ", folderRemove = "rmdir", folder, projectName, editor, className, path;

    const char* create = R"(
[0] Main Menu.
[1] New Project
[2] Exit
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
            case 2: exit(0); break;
            default: fmt::print("command: {} is unknown", ans); CreationOptions(); break;
        }
    };

    void path_write(){
        std::ofstream cpc("path.txt", std::ios::app);
        cpc << path << std::endl;
    };
    void project_write(){
        std::ofstream cpc("project.txt", std::ios::app);
        cpc << projectName << std::endl;
    }

    void NewProject(){
        fmt::print(fg(fmt::color::cyan), "> New Project! <\n");
        fmt::print(fg(fmt::color::golden_rod), "Project name: ");
        std::cin >> projectName;
        path = std::filesystem::current_path().string() + "\\" + projectName;
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