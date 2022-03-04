#pragma once
#include <iostream>

/**
 * @brief  All functions and input for the command line.
 * @param menu: const char* - Main Menu 
 * @param menuAnswer: int
 * @param help: const char*
 * @retval None
 */
namespace cmdInformation {

  const char *menu = R""""(  
[1] CP Create
[2] CP Projects
[3] CP Add
[4] Help
[5] exit
)"""";

  // answer for cmdInformation::menu
  int menuAnswer;

  const char *help = R""""(
[X] CP Create - Creates a new CP project.
[X] CP Projects - Shows all Projects made with CP
[X] CP Add - Adds new Files and Folders to existing CP Project.
[X] cp help - Displays this help message.
)"""";
}
/**
 * note: cpfd fixme: refractor code to fit function
 * */

void removeFolder(){ 
   const char *help = R""""()"""";

  std::string check, name, concat, cmd = "del ", scmd, nconcat;
  std::cout << "----------------CP Remover----------------" << std::endl;
  std::cout << "What do you want to delete? Options: File or Folder."
            << std::endl;
  std::cin >> check;
  if (check == "File" || check == "file") {
    std::cout << "What file do you want to delete?" << std::endl;
    std::cin >> name;
    concat = cmd + name;
    system(concat.c_str());
  } else if (check == "Folder" || check == "folder") {
    std::cout << "What folder do you want to delete?" << std::endl;
    std::cin >> name;
    scmd = "rmdir /s ";
    nconcat = scmd + name;
    system(nconcat.c_str());
  }

  std::cout << "Thank you for using CP Remover." << std::endl;
  system("cp"); 
}

