#include <iostream>
#include <windows.h>
#include "../headers/misc.h"
/*
CP

This is CP's main file.
Although its not much, its responsible for the recursion between options 2, 3, and 4;
If any new files are created, you want to add a new option and also add it to the switch.
*/

/**
 * CPFC
 * */

void cpfc(){ 
    std::string folderName, answer, folderCreate = "md ", folder; // string variables
	std::cout << "----------------CP Folder----------------" << std::endl;
	std::cout << "What is the name of your folder?" << std::endl;
	std::cin >> folderName; // input
	folder = folderCreate + folderName;
	std::system(folder.c_str()); // requires c_str() function to turn the string into a const char * for the system function
	std::cout << "Run the command, cd {folder name} and cpfile to create file. " << std::endl;
	std::cout << "Thank you for using CP Folder." << std::endl;
}

int main(){
    int option; // switch option
    std::cout << "Welcome to Creation Project. What would you like to do?: " << std::endl;

    std::cout << menu << std::endl;
    std::cin >> option; // input check. can be changed to only accept numbers and recursively run if a valid number is not given. 
    // todo: migrate all files here to main file
    // todo: remove executables and turn comand line calls into functions
    // todo: create something similar to cargo's `cargo new` function
    switch(option){
        case 1: system("cpfc"); break;
        case 2: system("tree /f"); system("cp"); break;
        case 3: system("cpfd"); break; 
        case 4: system("cpfile"); break;
        case 5: system("cpproject"); break;
        case 6: system("wiki"); break;
        case 7: std::cout << "Thank you for using CP File." << std::endl; return 0; break; // no default case
    }
}