#pragma once

#include <string.h>
#include <iostream>


namespace arguments {
  /**
 * @brief  Argument Check Function
 * @note   
 * @param  argc: int, takes main argc
 * @param  argv[]: int, takes main argv
 * @param  arguments: int, which argument it is at, 2> for 2 or more aruguments on program call
 * @param  argvParameter: int, index of argv, 1> for it to work
 * @param  word: sting, word that should be compared with argv
 * @retval true: if arguments are compared false: otherwise
 */
bool argCheck(int argc, char* argv[], int arguments, int argvParameter, const char* word){
    if ( (argc == arguments) && _stricmp( argv[argvParameter], word) == 0)
        return true;
    else return false;
}
}