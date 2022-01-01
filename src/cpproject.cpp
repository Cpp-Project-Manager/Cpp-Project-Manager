#include <iostream>
#include <windows.h>

void cppproject();

int main(){
    std::string ans;
    std::cout << "----------------CP Project Creator----------------" << std::endl;
    std::cout << "What Project would you like to configure? (C++, Rust)" << std::endl;
    std::cin >> ans;
    if (ans == "C++" || ans == "cpp" || ans == "c plus plus" || ans == "c++"){
        cppproject();
    }
    else if(ans == "Rust" || ans == "rs" || ans == "rust" || ans == "Rs"){
        std::cout << "What is the name of your project?" << std::endl;
        std::string projectName;
        std::cin >> projectName;
        std::string cargo = "cargo new ";
        std::string cmd = cargo + projectName;
        system(cmd.c_str());
    }
    std::cout << "Thank you for using Cp Project Creator." << std::endl;
    return 0;
}