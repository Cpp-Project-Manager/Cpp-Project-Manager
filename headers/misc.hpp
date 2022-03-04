#pragma once
#include <iostream>

/**
 * @brief  All Miscellaneous functions and input.
 * @param menu: const char* - Main Menu 
 * @param menuAnswer: int
 * @param help: const char*
 * @retval None
 */
namespace cmdInformation {

  const char *menu = R"(  
[1] CP Create
[2] CP Projects
[3] CP Add
[4] Help
[5] exit
)";

  // answer for cmdInformation::menu
  int menuAnswer;

  const char *help = R"(
[X] cp create - Creates a new CP project.
[X] cp projects - Shows all Projects made with CP
[X] cp add - Adds new Files and Folders to existing CP Project.
[X] cp help - Displays this help message.
)";

const char* projectTemplates = R"(
[1] Blank - No files Or Folders.
[2] Src - Main program with header.
[3] Class - Main with specified class.
[4] Skip
[5] Exit
)";
}

namespace conv {
  auto cppboiler = 
R"(#include <iostream>
#include "../include/main.hpp"

int main(){

	std::cout << "Hello World" << std::endl;
	return 0;

})";

}