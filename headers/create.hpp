#pragma once

#include <bits/stdc++.h>
#include <unistd.h>
#include <windows.h>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <simpio/simpio.h>

#include "../headers/conv.hpp"

// Info: So what create needs to do, is to create, ofc, c++ projects that can be opened stright from the command line anywhere. It also needs to list dependencies used in that project. Information can and will be stored in either a json file or NBT file and retrieved using either Nlohmann Json [https://github.com/nlohmann/json] or nbt-cpp [https://github.com/handtruth/nbt-cpp].
/**
 *  keypoints:
 * ? Can be opened in specified code editor from any command line
 * ! above needs to impliment a path handler
 * */

/**
 * @brief  Create Class: Relevant to Cp Create.
 * @note   
 * @retval None
 */
class Create {

    public:
    Create(){};
    void NewProject(){
        std::cout << "> New Project! <" << std::endl;
    };
};