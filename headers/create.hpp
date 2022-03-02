#pragma once

#include <bits/stdc++.h>
#include <unistd.h>
#include <windows.h>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <simpio/simpio.h>


#include "../headers/conv.hpp"
#include "../headers/json.hpp"

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
<<<<<<< HEAD
    int ans;
    std::string folderName, answer, folderCreate = "md ", folder;
    const char* create = R""""(
[1] New Project
[2] New File in current directory.
[3] New File in specified directory.
)"""";
    public:
    Create(){};
    void CreationOptions(){
        fmt::print(fg(fmt::terminal_color::cyan), "What do you want to create?");
        fmt::print(fg(fmt::color::dark_golden_rod), create);
        std::cin >> ans;
        switch(ans){ 
            case 0: fmt::print("Go back to main menu."); break;
            case 1: NewProject(); break;
            case 2: fmt::print("NewFile()"); break;
            case 3: fmt::print("NewFileDir()"); break;
            
        }
    };
    void NewProject(){
        std::cout << "> New Project! <" << std::endl;
    };
};

// 	std::cout << "What is the name of your folder?" << std::endl;
// 	std::cin >> folderName; // input
// 	folder = folderCreate + folderName;
// 	std::system(folder.c_str()); // requires c_str() function to turn the string into a const char * for the system function
// 	std::cout << "Run the command, cd {folder name} and cpfile to create file. " << std::endl;
// 	std::cout << "Thank you for using CP Folder." << std::endl;
//   system("cp");
// }
=======
    std::string projectName;

    public:
    Create(){};
    // Create(std::string pn) : projectName(pn){};

    void createFolder(std::string projectName){ 
        std::string folderCreate = "md ", folder; // can be mkdir
        folder = folderCreate + projectName;
        std::system(folder.c_str()); // requires c_str() function to turn the string into a const char * for the system function
        std::cout << "Run the command, cd {folder name} and cpfile to create file. " << std::endl;
        std::cout << "Thank you for using CP Folder." << std::endl;
        system("cp");
    }
    
    void NewProject(){
        std::cout << "> New Project! <" << std::endl;
        std::cout << "Project name: "; std::cin >> projectName; std::cout << std::endl;
        createFolder(projectName);
    }
};
>>>>>>> 922e56b77430ab269bf424895b0571c68c31b737
