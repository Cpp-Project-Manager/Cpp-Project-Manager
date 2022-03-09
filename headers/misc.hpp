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

  // answer for cmdInformation::menu
  int menuAnswer;

  const char *help = 
R"(
[X] cp create - Creates new project with your specifications.
[X] cp new    - {project} Creates a boiler plate project.
[X] cp help   - Displays this help message.
)";

const char* projectTemplates = R"(
[1] Blank - No files Or Folders.
[2] Src - Main program with header.
[3] Class - Main with specified class.
[4] Exit
)";
}

namespace conv {
  auto cppboiler = 
R"(#include <iostream>

int main(){

	std::cout << "Hello World" << std::endl;
	return 0;

})",
      headerBoiler = 
R"(#pragma once

#include <iostream>
)";

}