#pragma once

#include <string.h>
#include <iostream>


namespace arguments {
  /**
 * @brief  Argument Check Function
 * @note   
 * @param  argc: int, takes main argc
 * @param  argv[]: int, takes main argv
 * @param  arguments: int, the amount of keywords that are going to be called after the program name
 * @param  argvParameter: int, index of each word
 * @param  word: sting, word that should be compared with argv
 * @retval true: if arguments are compared false: otherwise
 */
bool argCheck(const char* word = "", int argc = NULL, char* argv[] = NULL, int arguments = 2, int argvParameter = 1){
    if ( (argc == arguments) && _stricmp( argv[argvParameter], word) == 0)
        return true;
    else return false;
}
}