#include <iostream>
#include <windows.h>
/*
CP

This is CP's main file.
Although its not much, its responsible for the recursion between options 2, 3, and 4;
If any new files are created, you want to add a new option and also add it to the switch.
*/

int main(){
    int option; // switch option
    std::cout << "Welcome to Creation Project. What would you like to do?: " << std::endl;
    const char *menu = R""""( 
1) CP Folder Creation
2) CP Folder/File List
3) CP Folder Removal
4) CP File Creation
5) CP Project Creation
6) Exit
    )""""; // const char C multi line string, it can be changed to a non constant char if need be. Warnings may be generatied.
    std::cout << menu << std::endl;
    std::cin >> option; // input check. can be changed to only accept numbers and recursively run if a valid number is not given. 
    switch(option){
        case 1:
        system("cpfc");
        break;
        case 2:
        system("tree /f");
        system("cp");
        break;
        case 3:
        system("cpfd");
        break; 
        case 4:
        system("cpfile");
        break;
        case 5:
        system("cpproject");
        break;
        case 6:
        std::cout << "Thank you for using CP File." << std::endl;
        return 0;
        break; // no default case
    }
}