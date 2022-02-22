#pragma once
#include <iostream>
/**
 * misc.h is a file meant for miscelaneous declarations/definitions needed. It can be migrated to main but code will be congested.
 * 
 * */

/**
 * @brief  All functions and 
 * @note   
 * @retval None
 */
namespace cmdInformation {
    // todo: change menu
  const char *menu = R""""(  
  > 1 CP Create
  > 2 CP Projects
  > 3 CP Add
  > 4 cp help
  > 5 exit
  )"""";

  const char *help = R""""(
All cp arguments can be called manually or called by using `cp`'s menu.
  > CP Create - Creates a new CP project.
  > CP Projects - Shows all Projects made with CP
  > CP Add - Adds new Files and Folders to existing CP Project.
  > cp help - Displays this help message.
  )"""";


}

/**
 * note: CPFC fixme: refractor code to fit function
 * */

void createFolder(){ 
    std::string folderName, answer, folderCreate = "md ", folder; // string variables
	std::cout << "----------------CP Folder----------------" << std::endl;
	std::cout << "What is the name of your folder?" << std::endl;
	std::cin >> folderName; // input
	folder = folderCreate + folderName;
	std::system(folder.c_str()); // requires c_str() function to turn the string into a const char * for the system function
	std::cout << "Run the command, cd {folder name} and cpfile to create file. " << std::endl;
	std::cout << "Thank you for using CP Folder." << std::endl;
  system("cp");
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

